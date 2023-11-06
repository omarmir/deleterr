import { defineStore } from 'pinia'
import { reactive, ref, Ref, watch } from 'vue'
import { APIResponse, RequestStatus, RequestStatusWithRecordInfo } from '~/@types/deleterr'

export const useRequestsStore = defineStore('requests', () => {
  const requests: Ref<RequestStatus[] | undefined> = ref([])
  const allRequests: Ref<number> = ref(0)
  const currentPage: Ref<number> = ref(0)
  const pageCount: Ref<number> = ref(0)
  const filteredRequests: Ref<number> = ref(0)
  const search: Ref<string> = ref('')

  const tableState: TableState = reactive({
    sortBy: 'requestedDate',
    isDescending: true,
    take: 5,
    skip: undefined,
    search: undefined,
  })

  const resort = (sorter: Sortables) => {
    if (tableState.sortBy === sorter) {
      tableState.isDescending = !tableState.isDescending
    } else {
      ;[tableState.sortBy, tableState.isDescending] = [sorter, false]
    }
  }

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

  watch(tableState, (newValue, oldValue) => {
    console.log(newValue)
    console.log(oldValue)
    console.log(tableState)
  })

  watch(search, (newValue, oldValue) => {
    console.log(newValue)
    console.log(oldValue)
    console.log(tableState)
  })

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
