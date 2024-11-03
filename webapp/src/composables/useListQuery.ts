import { useDebounce } from '@vueuse/core'
import { computed, ComputedRef, ref, Ref, watch } from 'vue'
import { ListQueryParams } from '~/@types/request-list'

export function useListQuery(params: { search: Ref<string | null>, take: number }) {

    const endpoint = '/api/v1/json/requests'

    const query: Ref<ListQueryParams> = ref({
        sortBy: 'requestedDate',
        isDescending: true,
        take: params.take,
        skip: 0,
        search: null
    })

    const debouncedSearch = useDebounce(params.search, 500)


    watch(debouncedSearch, (newSearch, oldSearch) => {
        if (newSearch === oldSearch) return

        query.value = { ...query.value, search: newSearch, take: params.take, skip: 0 }

    })

    const url: ComputedRef<string> = computed(() => {
        const queryString = Object.entries(query.value)
            .filter(([_key, value]) => value !== null && value !== undefined) // Filter out properties with undefined values
            .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value as string)}`)
            .join('&')


        return `${endpoint}?${queryString}`
    })

    const currentPage = computed(() => (query.value.skip ?? 0) / query.value.take)

    const changePage = (page: number) => (query.value.skip = page * query.value.take)

    const increment = () => changePage(currentPage.value + 1)

    const decrement = () => changePage(currentPage.value - 1)


    return {
        url,
        query,
        currentPage,
        changePage,
        increment,
        decrement
    }
}
