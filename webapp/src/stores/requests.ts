import { defineStore } from 'pinia'
import { reactive, readonly, ref, Ref, watch } from 'vue'
import { APIResponse, RequestStatus, RequestStatusWithRecordInfo } from '~/@types/deleterr'

export const useRequestsStore = defineStore('requests', () => {
  const requests: Ref<RequestStatus[] | undefined> = ref([])
  const allRequests: Ref<number> = ref(0)
  const currentPage: Ref<number> = ref(0)
  const pageCount: Ref<number> = ref(0)
  const filteredRequests: Ref<number> = ref(0)
  /**
   * Search needs to be managed as a seperate ref because there needs to be comparision
   * and reactive objects do not allow comparision from old to new value since it points
   * to the same place.
   */
  const search: Ref<string> = ref('')

  const tableState: TableState = reactive({
    sortBy: 'requestedDate',
    isDescending: true,
    take: 5,
    skip: undefined,
    search: undefined,
  })

  // TODO: Have this be read only so that commands are sent instead of binding directly to tablestate
  const copy: Readonly<TableState> = readonly(tableState)

  const getRequests = async () => {
    try {
      const queryString = Object.entries(tableState)
        .filter(([_key, value]) => value !== undefined) // Filter out properties with undefined values
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
        .join('&')

      const urlWithQueryParams = `http://localhost:8080/api/v1/json/requests?${queryString}`

      const response = await fetch(urlWithQueryParams)
      let apiResponse: APIResponse<RequestStatusWithRecordInfo> = await response.json()
      requests.value = apiResponse.data?.requests
      allRequests.value = apiResponse.data?.allRequests ?? 0
      currentPage.value = (tableState.skip ?? 0) / tableState.take
      filteredRequests.value = apiResponse.data?.filteredRequests ?? 0
      pageCount.value = Math.ceil(filteredRequests.value / tableState.take)
    } catch (error) {
      console.error(error)
    }
  }

  watch(tableState, (_newState: TableState) => getRequests())

  watch(search, (newValue, oldValue) => {
    if (newValue != oldValue) {
      [tableState.search, tableState.skip] = [newValue, 0]
      currentPage.value = 0
    }
  })

  const resort = (sorter: Sortables) => {
    if (tableState.sortBy === sorter) {
      tableState.isDescending = !tableState.isDescending
    } else {
      ;[tableState.sortBy, tableState.isDescending] = [sorter, false]
    }
  }

  const changePage = (page: number) => {
    tableState.skip = page * tableState.take
  }

  return {
    resort,
    requests,
    allRequests,
    tableState,
    getRequests,
    currentPage,
    pageCount,
    changePage,
    filteredRequests,
    search,
  }
})
