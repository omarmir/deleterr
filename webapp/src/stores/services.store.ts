import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { OperationState } from '~/@types/common'
import { APIResponse, ServiceInfo, Services } from '~/@types/deleterr'
import { AllServiceStatus, ServiceOperations } from '~/@types/services'

export const useServiceStore = defineStore('services', () => {
  const serviceStatus: Ref<AllServiceStatus> = ref({
    overseerr: { test: OperationState.hidden, save: OperationState.hidden, errorMsg: '' },
    tautulli: { test: OperationState.hidden, save: OperationState.hidden, errorMsg: '' },
    radarr: { test: OperationState.hidden, save: OperationState.hidden, errorMsg: '' },
    sonarr: { test: OperationState.hidden, save: OperationState.hidden, errorMsg: '' },
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
      credentials: 'include',
    }

    const apiEndpoint =
      operation === ServiceOperations.Save ? '/api/v1/json/service/save' : '/api/v1/json/service/status'

    try {
      serviceStatus.value[serviceInfo.service][operation] = OperationState.loading
      const response = await fetch(apiEndpoint, requestOptions)
      let apiResponse: APIResponse<ServiceInfo> = await response.json()

      if (apiResponse.success) {
        serviceStatus.value[serviceInfo.service][operation] = OperationState.success
        setTimeout(() => {
          serviceStatus.value[serviceInfo.service][operation] = OperationState.hidden
        }, 5000)
      } else {
        serviceStatus.value[serviceInfo.service][operation] = OperationState.failure
        serviceStatus.value[serviceInfo.service].errorMsg = apiResponse.error_msg ?? ''
      }
    } catch (error: any) {
      console.error(error)
      serviceStatus.value[serviceInfo.service][operation] = OperationState.failure
      serviceStatus.value[serviceInfo.service].errorMsg = error
    }
  }

  return { saveService, testService, getServices, services, serviceStatus }
})
