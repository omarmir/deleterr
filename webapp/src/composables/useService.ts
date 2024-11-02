import { useFetch } from "@vueuse/core"
import { computed, reactive, watch } from "vue"
import { OperationState } from "~/@types/common"
import { APIResponse, ServiceInfo, Services } from "~/@types/deleterr"
import { useToast } from "./useToast"

export function useService(serviceType: Services, serviceInfo?: ServiceInfo) {

    const { publishToast } = useToast()

    const service = reactive({
        host: serviceInfo?.host ?? '',
        apiKey: serviceInfo?.apiKey ?? '',
        port: serviceInfo?.port ?? '',
        useSsl: serviceInfo?.useSsl ?? false,
        service: serviceType,
    })

    const {
        error: testError,
        data: testData,
        execute: test,
        isFetching: isTesting,
    } = useFetch('/api/v1/json/service/status', {
        immediate: false,
        refetch: true
    })
        .post(() => JSON.stringify(service), 'application/json')
        .json<APIResponse<String>>()

    const {
        error: saveError,
        data: saveData,
        execute: save,
        isFetching: isSaving,
    } = useFetch('/api/v1/json/service/save', {
        immediate: false,
    })
        .post(() => JSON.stringify(service), 'application/json')
        .json<APIResponse<String>>()


    const testingState = computed(() => {
        if (isTesting.value) return OperationState.loading
        if (testData.value?.success) return OperationState.success
        if (testError.value || testData.value?.success === false) return OperationState.failure
        return OperationState.hidden
    })

    const savingState = computed(() => {
        if (isSaving.value) return OperationState.loading
        if (saveData.value?.success) return OperationState.success
        if (saveError.value || saveData.value?.success === false) return OperationState.failure
        return OperationState.hidden
    })

    watch([saveError, saveData], () => {
        if (saveError.value) {
            publishToast('Unable to save service', 'Error: ' + (saveError.value as any).toString(), 10, true)
        } else if (!saveData.value?.success) {
            publishToast('Unable to save service', 'Error: ' + saveData.value?.error_msg, 10, true)
        }
    })

    watch([testError, testData], () => {
        if (testError.value) {
            publishToast('Unable to save service', 'Error: ' + (testError.value as any).toString(), 10, true)
        } else if (!testData.value?.success) {
            publishToast('Unable to save service', 'Error: ' + testData.value?.error_msg, 10, true)
        }
    })

    return {
        test,
        save,
        testingState,
        savingState,
        service
    }
}
