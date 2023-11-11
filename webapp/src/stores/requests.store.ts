import { defineStore } from 'pinia'
import { reactive, readonly, ref, Ref, watch } from 'vue'
import {
  APIResponse,
  MediaExemption,
  RequestStatus,
  RequestStatusWithRecordInfo,
  SingleMediaExeption,
  TestState,
} from '~/@types/deleterr'

export const useRequestsStore = defineStore('requests', () => {
  const requests: Ref<RequestStatus[] | undefined> = ref([])
  const allRequests: Ref<number> = ref(0)
  const currentPage: Ref<number> = ref(0)
  const pageCount: Ref<number> = ref(0)
  const filteredRequests: Ref<number> = ref(0)
  const error: Ref<any | null> = ref(null)
  const mediaExemptions: Ref<MediaExemption> = ref({})
  const exemptionsActionState: Ref<{ [key: number]: TestState }> = ref({})

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
    exemptionsActionState.value[key] = TestState.loading

    try {
      const response = await fetch(mediaExemptionsEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()

      if (apiResponse.success) {
        mediaExemptions.value[key] = mediaExemption[1]
        exemptionsActionState.value[key] = TestState.success
      } else {
        console.log('Error: ' + apiResponse.error_msg)
        exemptionsActionState.value[key] = TestState.failure
        setTimeout(() => {
          exemptionsActionState.value[key] = TestState.hidden
        }, 5000)
      }
    } catch (err) {
      console.error(err)
      exemptionsActionState.value[key] = TestState.failure
      setTimeout(() => {
        exemptionsActionState.value[key] = TestState.hidden
      }, 5000)
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
    exemptionsActionState.value[key] = TestState.loading

    try {
      const response = await fetch(mediaExemptionsEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()
      if (apiResponse.success) {
        delete mediaExemptions.value[key]
        exemptionsActionState.value[key] = TestState.success
      } else {
        console.log('Error: ' + apiResponse.error_msg)
        exemptionsActionState.value[key] = TestState.failure
        setTimeout(() => {
          exemptionsActionState.value[key] = TestState.hidden
        }, 5000)
      }
    } catch (err) {
      console.error(err)
      exemptionsActionState.value[key] = TestState.failure
      setTimeout(() => {
        exemptionsActionState.value[key] = TestState.hidden
      }, 5000)
    }
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
    exemptionsActionState,
    // Commands
    search,
    resort,
    changePage,
    addMediaExemption,
    removeMediaExemption,
  }
})
