import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { APIResponse, ServiceInfo, ServiceOperations, TestState, AllServiceStatus, Services } from '~/@types/deleterr'

export const useServiceStore = defineStore('services', () => {

    const serviceStatus: Ref<AllServiceStatus> = ref({
        'overseerr': { test: TestState.hidden, save: TestState.hidden, errorMsg: '' },
        'tautulli': { test: TestState.hidden, save: TestState.hidden, errorMsg: '' },
        'radarr': { test: TestState.hidden, save: TestState.hidden, errorMsg: '' },
        'sonarr': { test: TestState.hidden, save: TestState.hidden, errorMsg: '' }
    })

    const services = ref<Record<Services, ServiceInfo> | undefined>(undefined)

    const saveService = async (serviceInfo: ServiceInfo) => saveTestService(serviceInfo, ServiceOperations.Save)
    const testService = async (serviceInfo: ServiceInfo) => saveTestService(serviceInfo, ServiceOperations.Test)


    const getServices = async () => {
        try {
            const response = await fetch('/api/v1/json/service/get', { credentials: 'include' })
            let apiResponse: APIResponse<Record<Services, ServiceInfo>> = await response.json()
            services.value = apiResponse.data
        } catch (error) {
            console.error(error)
        }
    }

    const saveTestService = async (serviceInfo: ServiceInfo, operation: ServiceOperations) => {
        const requestOptions: RequestInit = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(serviceInfo),
            credentials: 'include'
        }

        const apiEndpoint =
            operation === ServiceOperations.Save
                ? '/api/v1/json/service/save'
                : '/api/v1/json/service/status'

        try {
            serviceStatus.value[serviceInfo.service][operation] = TestState.loading
            const response = await fetch(apiEndpoint, requestOptions)
            let apiResponse: APIResponse<ServiceInfo> = await response.json()

            if (apiResponse.success) {
                serviceStatus.value[serviceInfo.service][operation] = TestState.success
                setTimeout(() => {
                    serviceStatus.value[serviceInfo.service][operation] = TestState.hidden
                }, 5000)
            } else {
                serviceStatus.value[serviceInfo.service][operation] = TestState.failure
                serviceStatus.value[serviceInfo.service].errorMsg = apiResponse.error_msg ?? ''
            }
        } catch (error: any) {
            console.error(error)
            serviceStatus.value[serviceInfo.service][operation] = TestState.failure
            serviceStatus.value[serviceInfo.service].errorMsg = error
        }
    }

    return { saveService, testService, getServices, services, serviceStatus }
})
