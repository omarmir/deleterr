import { useFetch } from "@vueuse/core"
import { computed, watch } from "vue"
import { OperationState } from "~/@types/common"
import { APIResponse, SeriesDeletionRequest } from "~/@types/deleterr"
import { useToast } from "./useToast"

export function useDeleteSeries(requestId: number) {

    const { publishToast } = useToast()

    const {
        error: errorWatched,
        data: dataWatched,
        execute: deleteWatched,
        isFetching: isFetchingWatched,
    } = useFetch(`/api/v1/json/series/${requestId}/delete/seasons/watched`, {
        immediate: false,
        timeout: 6000000
    })
        .delete()
        .json<APIResponse<SeriesDeletionRequest>>()

    const deletingWatchedState = computed(() => {
        if (isFetchingWatched.value) return OperationState.loading
        if (dataWatched.value?.success) {
            if (dataWatched.value.data?.cacheResponse.success && dataWatched.value.data?.overseerrResponse?.success && dataWatched.value.data?.sonarrResponse?.success) {
                return OperationState.success
            } else {
                return OperationState.failure
            }
        }
        if (errorWatched.value || errorWatched.value?.success === false) return OperationState.failure
        return OperationState.hidden
    })

    watch([errorWatched, dataWatched, dataWatched], () => {
        if (errorWatched.value) {
            publishToast('Unable to delete series', 'Error: ' + (errorWatched.value as any).toString(), 10, true)
        } else if (!dataWatched.value?.success) {
            publishToast('Unable to delete series', 'Error: ' + dataWatched.value?.error_msg, 10, true)
        } else if (dataWatched.value.success) {
            if (!dataWatched.value.data?.sonarrResponse) {
                publishToast('Unable to delete series', `Unable to delete series in Sonarr. Status code: ${dataWatched.value.data?.sonarrResponse?.status}`, 10, true)
                return
            }
            if (!dataWatched.value.data?.cacheResponse.success) publishToast('Unable to fully remove series', 'Unable to remove from cache', 10, true)
            if (!dataWatched.value.data?.overseerrResponse?.success) publishToast('Unable to fully remove series', 'Unable to remove from overseerr', 10, true)

            if (dataWatched.value.data?.cacheResponse.success && dataWatched.value.data?.overseerrResponse?.success && dataWatched.value.data?.sonarrResponse?.success) {
                publishToast('Deleted watched seasons', 'Watched seasons have been delete, the show still exists in Sonarr any new seaons will be downloaded', 10, false)

            }
        }
    })

    return {
        deleteWatched,
        deletingWatchedState,
        dataWatched
    }
}
