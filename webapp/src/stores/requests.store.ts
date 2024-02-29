import { defineStore } from 'pinia'
import { reactive, readonly, ref, Ref, watch } from 'vue'
import {
  APIResponse,
  MediaExemptions,
  RequestStatus,
  RequestStatusWithRecordInfo,
  MovieDeletionRequest,
} from '~/@types/deleterr'
import { useToast } from '~/composables/useToast'
import { useRouter } from 'vue-router'
interface RequestResponse {
  requests?: RequestStatus[]
  exemptions?: MediaExemptions
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
  const mediaExemptions: Ref<MediaExemptions> = ref([])

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

    // Use Promise.all to run both promises in parallel*
    const [requestsResponse, mediaExemptionsResponse] = await Promise.all([
      fetch(urlWithQueryParams, {
        credentials: 'include',
      }),
      fetch(mediaExemptionsEndpoint, {
        credentials: 'include',
      }),
    ])

    // Use Promise.all again to extract JSON from the responses in parallel*
    const [requestsResult, mediaExemptionsResult] = await Promise.all([
      requestsResponse.json() as Promise<APIResponse<RequestStatusWithRecordInfo>>,
      mediaExemptionsResponse.json() as Promise<APIResponse<MediaExemptions>>,
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
      mediaExemptions.value = resp?.exemptions ?? []
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

  const isMediaExempted = (requestId: number) => {
    return mediaExemptions.value.includes(requestId)
  }

  const toggleMediaExemption = async (mediaExemption: number): Promise<APIResponse<string> | undefined> => {
    return isMediaExempted(mediaExemption)
      ? await removeMediaExemption(mediaExemption)
      : await addMediaExemption(mediaExemption)
  }

  const addMediaExemption = async (mediaExemption: number): Promise<APIResponse<string> | undefined> => {
    const mediaExemptionsEndpoint = '/api/v1/json/request/exemptions/save'

    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(mediaExemption),
      credentials: 'include',
    }

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaExemptionsEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()

      if (apiResponse.success) {
        publishToast('Exempted', 'This media item will not be automatically deleted at next scheduled run.', 3, false)
        mediaExemptions.value.push(mediaExemption)
        return apiResponse
      } else {
        publishToast('Unable to add exemption', 'Error: ' + apiResponse.error_msg ?? 'Unknown!', 3, true)
        return apiResponse
      }
    } catch (err: any) {
      const apiResponse: APIResponse<string> = {
        success: false,
        error_msg: err
      }
      publishToast('Unable to add exemption', 'Error: ' + err.toString(), 10, true)
      return apiResponse
    }
  }

  const removeMediaExemption = async (mediaExemption: number): Promise<APIResponse<string> | undefined> => {
    const mediaExemptionsEndpoint = '/api/v1/json/request/exemptions/remove'

    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(mediaExemption),
      credentials: 'include',
    }

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaExemptionsEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()

      if (apiResponse.success) {
        publishToast(
          'Exemption removed',
          'This media item will be automatically deleted at the next scheduled run.',
          3,
          true
        )
        mediaExemptions.value = mediaExemptions.value.filter((exemption) => exemption != mediaExemption)
        return apiResponse
      } else {
        publishToast('Unable to remove exemption', 'Error: ' + apiResponse.error_msg ?? 'Unknown!', 3, true)
        return apiResponse
      }
    } catch (err: any) {
      const apiResponse: APIResponse<string> = {
        success: false,
        error_msg: err
      }
      publishToast('Unable to remove exemption', 'Error: ' + err.toString(), 10, true)
      return apiResponse
    }
  }

  const deleteMovieFile = async (requestId: number): Promise<APIResponse<MovieDeletionRequest>> => {
    const mediaDeleteEndpoint = `/api/v1/json/movie/delete/${requestId}`
    const requestOptions: RequestInit = {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
    }

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(mediaDeleteEndpoint, requestOptions)
      let apiResponse: APIResponse<MovieDeletionRequest> = await response.json()
      if (apiResponse.success) {
        publishToast(
          'Movie deleted',
          'Movie has been deleted! You may need to re-scan on plex for it to vanish',
          5,
          false
        )
        getRequests() //TODO: We may wish to remove it locally instead
        return apiResponse
      } else {
        publishToast('Unable to delete movie', 'Error: ' + apiResponse.error_msg ?? 'Unknown!', 3, true)
        return apiResponse
      }
    } catch (err: any) {
      const apiResponse: APIResponse<MovieDeletionRequest> = {
        success: false,
        error_msg: err.toString()
      }
      publishToast('Unable to delete movie', 'Error: ' + (err as any).toString(), 10, true)
      return apiResponse
    }
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
    isMediaExempted,
    // Commands
    search,
    resort,
    changePage,
    deleteMovieFile,
    toggleMediaExemption,
  }
})
