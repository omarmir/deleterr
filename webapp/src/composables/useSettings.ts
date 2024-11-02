import { useFetch } from "@vueuse/core"
import { computed, reactive, watch } from "vue"
import { OperationState } from "~/@types/common"
import { APIResponse, Settings } from "~/@types/deleterr"
import { useToast } from "./useToast"

export function useSettings(initialSettings?: Settings) {

    const { publishToast } = useToast()

    const settings = reactive(initialSettings ?? {
        tvPurgeMarker: 'watched',
        tvWatchedMarker: 'watched',
        tvPurgeDelay: undefined,
        tvPurgeStrategy: 'season',
        moviePurgeMarker: 'watched',
        moviePurgeDelay: undefined
    })

    const {
        error: saveError,
        data: saveData,
        execute: save,
        isFetching: isSaving,
    } = useFetch('/api/v1/json/settings/save', {
        immediate: false,
    })
        .post(() => JSON.stringify(settings), 'application/json')
        .json<APIResponse<Settings>>()

    const savingState = computed(() => {
        if (isSaving.value) return OperationState.loading
        if (saveData.value?.success) return OperationState.success
        if (saveError.value || saveData.value?.success === false) return OperationState.failure
        return OperationState.hidden
    })

    watch([saveError, saveData], () => {
        if (saveError.value) {
            publishToast('Unable to save service', 'Error: ' + (saveError as any).toString(), 10, true)
        } else if (!saveData.value?.success) {
            publishToast('Unable to save service', 'Error: ' + saveData.value?.error_msg, 10, true)
        }
    })

    return {
        save,
        settings,
        savingState,
    }
}
