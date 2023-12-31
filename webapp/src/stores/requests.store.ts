import { defineStore } from 'pinia'
import { reactive, readonly, ref, Ref, watch } from 'vue'
import {
  APIResponse,
  MediaExemption,
  RequestStatus,
  RequestStatusWithRecordInfo,
  SingleMediaExeption,
  MovieDeletionRequest,
} from '~/@types/deleterr'
import { useToast } from '~/composables/useToast'
import { useRouter } from 'vue-router'
import { OperationState } from '~/@types/common'

interface RequestResponse {
  requests?: RequestStatus[]
  exemptions?: MediaExemption
  allRequests?: number
  filteredRequests?: number
}

export const useRequestsStore = defineStore('requests', () => {
  const requests: Ref<RequestStatus[] | undefined> = ref([])
  const allRequests: Ref<number> = ref(0)
  const currentPage: Ref<number> = ref(0)
  const pageCount: Ref<number> = ref(0)
  const filteredRequests: Ref<number> = ref(0)
  const error: Ref<any | null> = ref(null)
  const mediaExemptions: Ref<MediaExemption> = ref({})
  // We are going to create a hashmap here with "exemption_<key>" or "deletion_<key>" as the key and the state as value.
  const actionStates: Ref<{ [key: string]: OperationState }> = ref({})

  const { publishToast } = useToast()

  const router = useRouter()

  const internalTableState: TableState = reactive({
    sortBy: 'requestedDate',
    isDescending: true,
    take: 5,
    skip: undefined,
    search: undefined,
  })

  const tableState: Readonly<TableState> = readonly(internalTableState)

  const makeApiCallForRequests = async (): Promise<RequestResponse | undefined> => {
    const queryString = Object.entries(tableState)
      .filter(([_key, value]) => value !== undefined) // Filter out properties with undefined values
      .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
      .join('&')

    const urlWithQueryParams = `/api/v1/json/requests?${queryString}`
    const mediaExemptionsEndpoint = '/api/v1/json/request/exemptions/get'

    // Use Promise.all to run both promises in parallel
    const [requestsResponse, mediaExemptionsResponse] = await Promise.all([
      fetch(urlWithQueryParams, {
        credentials: 'include',
      }),
      fetch(mediaExemptionsEndpoint, {
        credentials: 'include',
      }),
    ])

    // Use Promise.all again to extract JSON from the responses in parallel
    const [requestsResult, mediaExemptionsResult] = await Promise.all([
      requestsResponse.json() as Promise<APIResponse<RequestStatusWithRecordInfo>>,
      mediaExemptionsResponse.json() as Promise<APIResponse<MediaExemption>>,
    ])

    if (!requestsResult.success || !mediaExemptionsResult.success) {
      const errorMsg = [requestsResult.error_msg, mediaExemptionsResult.error_msg].join(' ')
      publishToast('Unable to load requests', 'Error: ' + errorMsg, 10, true)
    }

    return {
      requests: requestsResult.data?.requests,
      exemptions: mediaExemptionsResult.data,
      allRequests: requestsResult.data?.allRequests,
      filteredRequests: requestsResult.data?.filteredRequests,
    }
  }

  const getRequests = async () => {
    try {
      const resp = await makeApiCallForRequests()
      requests.value = resp?.requests ?? []
      mediaExemptions.value = resp?.exemptions ?? {}
      allRequests.value = resp?.allRequests ?? 0
      filteredRequests.value = resp?.filteredRequests ?? 0
      currentPage.value = (tableState.skip ?? 0) / tableState.take
      pageCount.value = Math.ceil(filteredRequests.value / tableState.take)
      error.value = null
    } catch (err) {
      error.value = err
      publishToast('Unable to load requests', 'Is the application server running?', 10, true)
    }
  }

  const isMediaExempted = (requestId?: number) => {
    return mediaExemptions.value.hasOwnProperty(requestId ?? 0)
  }

  const toggleMediaExemption = async (mediaExemption: SingleMediaExeption) => {
    isMediaExempted(mediaExemption[0])
      ? await removeMediaExemption(mediaExemption)
      : await addMediaExemption(mediaExemption)
  }

  const addMediaExemption = async (mediaExemption: SingleMediaExeption) => {
    const mediaExemptionsEndpoint = '/api/v1/json/request/exemptions/save'

    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(mediaExemption),
      credentials: 'include',
    }

    //{ credentials: 'include', method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(mediaExemption) }

    const key = mediaExemption[0]
    actionStates.value['exemption_' + key] = OperationState.loading

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaExemptionsEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()

      if (apiResponse.success) {
        mediaExemptions.value[key] = mediaExemption[1]
        actionStates.value['exemption_' + key] = OperationState.success
        publishToast('Exempted', 'This media item will not be automatically deleted at next scheduled run.', 3, false)
      } else {
        handleErrorForExemption('exemption_' + key, apiResponse.error_msg)
      }
    } catch (err) {
      handleErrorForExemption('exemption_' + key, (err as any).toString())
    }
  }

  const removeMediaExemption = async (mediaExemption: SingleMediaExeption) => {
    const mediaExemptionsEndpoint = '/api/v1/json/request/exemptions/remove'

    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(mediaExemption[0]),
      credentials: 'include',
    }

    const key = mediaExemption[0]
    actionStates.value['exemption_' + key] = OperationState.loading

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaExemptionsEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()
      if (apiResponse.success) {
        delete mediaExemptions.value[key]
        actionStates.value['exemption_' + key] = OperationState.success
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
    const mediaDeleteEndpoint = `/api/v1/json/movie/delete/${requestId}`
    const requestOptions: RequestInit = {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
    }

    actionStates.value['delete_' + requestId] = OperationState.loading

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaDeleteEndpoint, requestOptions)
      let apiResponse: APIResponse<MovieDeletionRequest> = await response.json()
      if (apiResponse.success) {
        actionStates.value['delete_' + requestId] = OperationState.success
        publishToast(
          'Movie deleted',
          'Movie has been deleted! You may need to re-scan on plex for it to vanish',
          5,
          false
        )
        getRequests()
      } else {
        handleErrorForExemption(
          'delete_' + requestId,
          apiResponse.error_msg ?? apiResponse.error_msg ?? 'Unknown error'
        )
      }
    } catch (err) {
      handleErrorForExemption('delete_' + requestId, (err as any).toString())
    }
  }

  const handleErrorForExemption = (key: string, errorMsg?: string) => {
    publishToast('Exemption removed', 'Error: ' + errorMsg ?? 'Unknown!', 3, true)
    actionStates.value[key] = OperationState.failure
    setTimeout(() => {
      actionStates.value[key] = OperationState.hidden
    }, 5000)
  }

  watch(tableState, (_newState: TableState) => getRequests())

  const search = (evt: InputEvent) => {
    const searchStr = (evt.target as any).value
    if (searchStr != internalTableState.search) {
      ;[internalTableState.search, internalTableState.skip] = [searchStr, 0]
      currentPage.value = 0
      // If searching from another page, take them to the main page to see results
      if (router.currentRoute.value.path != '/') router.push('/')
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
    isMediaExempted,
    // Commands
    search,
    resort,
    changePage,
    deleteMovieFile,
    toggleMediaExemption,
  }
})
