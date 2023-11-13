import { defineStore } from 'pinia'
import { reactive, readonly, ref, Ref, watch } from 'vue'
import {
  APIResponse,
  MediaExemption,
  RequestStatus,
  RequestStatusWithRecordInfo,
  SingleMediaExeption,
  TestState,
  MovieDeletionRequest,
} from '~/@types/deleterr'

import { useToast } from '~/composables/useToast'

export const useRequestsStore = defineStore('requests', () => {
  const requests: Ref<RequestStatus[] | undefined> = ref([])
  const allRequests: Ref<number> = ref(0)
  const currentPage: Ref<number> = ref(0)
  const pageCount: Ref<number> = ref(0)
  const filteredRequests: Ref<number> = ref(0)
  const error: Ref<any | null> = ref(null)
  const mediaExemptions: Ref<MediaExemption> = ref({})
  // We are going to create a hashmap here with "exemption_<key>" or "deletion_<key>" as the key and the state as value.
  const actionStates: Ref<{ [key: string]: TestState }> = ref({})

  const { publishToast } = useToast()

  const internalTableState: TableState = reactive({
    sortBy: 'requestedDate',
    isDescending: true,
    take: 5,
    skip: undefined,
    search: undefined,
  })

  const tableState: Readonly<TableState> = readonly(internalTableState)

  const getRequests = async () => {
    try {
      const queryString = Object.entries(tableState)
        .filter(([_key, value]) => value !== undefined) // Filter out properties with undefined values
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
        .join('&')

      const urlWithQueryParams = `http://localhost:8080/api/v1/json/requests?${queryString}`
      const mediaExemptionsEndpoint = 'http://localhost:8080/api/v1/json/request/exemptions/get'

      // Use Promise.all to run both promises in parallel
      const [requestsResponse, mediaExemptionsResponse] = await Promise.all([
        fetch(urlWithQueryParams),
        fetch(mediaExemptionsEndpoint),
      ])

      // Use Promise.all again to extract JSON from the responses in parallel
      const [requestsResult, mediaExemptionsResult] = await Promise.all([
        requestsResponse.json() as Promise<APIResponse<RequestStatusWithRecordInfo>>,
        mediaExemptionsResponse.json() as Promise<APIResponse<MediaExemption>>,
      ])

      requests.value = requestsResult.data?.requests
      mediaExemptions.value = mediaExemptionsResult.data ?? {}
      allRequests.value = requestsResult.data?.allRequests ?? 0
      currentPage.value = (tableState.skip ?? 0) / tableState.take
      filteredRequests.value = requestsResult.data?.filteredRequests ?? 0
      pageCount.value = Math.ceil(filteredRequests.value / tableState.take)
      error.value = null
    } catch (err) {
      console.error(err)
      error.value = err
    }
  }

  const addMediaExemption = async (mediaExemption: SingleMediaExeption) => {
    const mediaExemptionsEndpoint = 'http://localhost:8080/api/v1/json/request/exemptions/save'

    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(mediaExemption),
    }

    const key = mediaExemption[0]
    actionStates.value['exemption_' + key] = TestState.loading

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaExemptionsEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()

      if (apiResponse.success) {
        mediaExemptions.value[key] = mediaExemption[1]
        actionStates.value['exemption_' + key] = TestState.success
        publishToast('Exempted', 'This media item will not be automatically deleted at next scheduled run.', 3, false)
      } else {
        handleErrorForExemption('exemption_' + key, apiResponse.error_msg)
      }
    } catch (err) {
      handleErrorForExemption('exemption_' + key, (err as any).toString())
    }
  }

  const removeMediaExemption = async (mediaExemption: SingleMediaExeption) => {
    const mediaExemptionsEndpoint = 'http://localhost:8080/api/v1/json/request/exemptions/remove'

    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(mediaExemption[0]),
    }

    const key = mediaExemption[0]
    actionStates.value['exemption_' + key] = TestState.loading

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaExemptionsEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()
      if (apiResponse.success) {
        delete mediaExemptions.value[key]
        actionStates.value['exemption_' + key] = TestState.success
        publishToast(
          'Exemption removed',
          'This media item will be automatically deleted at the next scheduled run.',
          3,
          true
        )
      } else {
        handleErrorForExemption('exemption_' + key, apiResponse.error_msg)
      }
    } catch (err) {
      handleErrorForExemption('exemption_' + key, (err as any).toString())
    }
  }

  const deleteMovieFile = async (requestId: number) => {
    const mediaDeleteEndpoint = `http://localhost:8080/api/v1/json/movie/delete/${requestId}`
    const requestOptions = {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' },
    }

    actionStates.value['delete_' + requestId] = TestState.loading

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaDeleteEndpoint, requestOptions)
      let apiResponse: APIResponse<MovieDeletionRequest> = await response.json()
      if (apiResponse.success) {
        requests.value = requests.value?.filter((req) => req.mediaRequest.id !== requestId)
        actionStates.value['delete_' + requestId] = TestState.success
        publishToast(
          'Movie deleted',
          'Movie has been deleted! You may need to re-scan on plex for it to vanish',
          3,
          true
        )
      } else {
        if (apiResponse.data?.radarrResponse) {
          handleErrorForExemption(
            'delete_' + requestId,
            'Deleted file in radarr but was unable to delete in Overseerr! Manually delete it there. Error: ' +
              apiResponse.error_msg ?? 'Unknown error'
          )
        } else {
          handleErrorForExemption(
            'delete_' + requestId,
            apiResponse.error_msg ?? apiResponse.error_msg ?? 'Unknown error'
          )
        }
      }
    } catch (err) {
      handleErrorForExemption('delete_' + requestId, (err as any).toString())
    }
  }

  const handleErrorForExemption = (key: string, errorMsg?: string) => {
    publishToast('Exemption removed', 'Error: ' + errorMsg ?? 'Unknown!', 3, true)
    actionStates.value[key] = TestState.failure
    setTimeout(() => {
      actionStates.value[key] = TestState.hidden
    }, 5000)
  }

  watch(tableState, (_newState: TableState) => getRequests())

  const search = (evt: InputEvent) => {
    const searchStr = (evt.target as any).value
    if (searchStr != internalTableState.search) {
      ;[internalTableState.search, internalTableState.skip] = [searchStr, 0]
      currentPage.value = 0
    }
  }

  const resort = (sorter: Sortables) => {
    if (tableState.sortBy === sorter) {
      internalTableState.isDescending = !internalTableState.isDescending
    } else {
      ;[internalTableState.sortBy, internalTableState.isDescending] = [sorter, false]
    }
  }

  const changePage = (page: number) => {
    internalTableState.skip = page * internalTableState.take
  }

  return {
    // States
    tableState,
    requests,
    allRequests,
    getRequests,
    currentPage,
    pageCount,
    filteredRequests,
    error,
    mediaExemptions,
    actionStates,
    // Commands
    search,
    resort,
    changePage,
    addMediaExemption,
    removeMediaExemption,
    deleteMovieFile,
  }
})
