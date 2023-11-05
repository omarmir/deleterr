import { reactive, ref, watch } from 'vue'
import type { Ref } from 'vue'
import { APIResponse, RequestStatus, RequestStatusWithRecordInfo } from '~/@types/deleterr'

type Sortables = 'name' | 'requestedDate' | 'mediaType' | 'watched' | 'user'

interface TableState {
    sortBy?: Sortables
    isDescending?: boolean
    take?: number
    skip?: number
    search?: string
}

const tableState: TableState = reactive({
    sortBy: 'requestedDate',
    isDescending: true,
    take: 5,
    skip: undefined,
    search: undefined,
})

export function useRequests() {
    const requests: Ref<RequestStatus[] | undefined> = ref([])
    const allRequests: Ref<number> = ref(0)

    const resort = (sorter: Sortables) => {
        if (tableState.sortBy === sorter) {
            tableState.isDescending = !tableState.isDescending
        } else {
            ;[tableState.sortBy, tableState.isDescending] = [sorter, false]
        }
    }

    watch(tableState, (_newState: TableState) => getRequests())

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
        } catch (error) {
            console.error(error)
        }
    }

    return {
        resort,
        requests,
        allRequests,
        tableState,
        getRequests
    }
}