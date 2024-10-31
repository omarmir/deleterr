import { computed, Ref } from 'vue'

export function useQueryURL(params: { endpoint: string, queryParams: Record<string, Ref<any>> }) {

    const url = computed(() => {
        const queryString = Object.entries(params.queryParams)
            .filter(([_key, value]) => value.value !== null && value.value !== undefined) // Filter out properties with undefined values
            .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value.value)}`)
            .join('&')

        return `${params.endpoint}?${queryString}`
    })

    return {
        url
    }
}
