import { defineStore } from 'pinia'
import { ref } from 'vue'
import { APIResponse, ServiceInfo, Services } from '~/@types/deleterr'
import { ServiceStatus } from '~/@types/services'
import { useToast } from '~/composables/useToast'

export const useServiceStore = defineStore('services', () => {
  const services = ref<Record<Services, ServiceInfo> | undefined>(undefined)
  const { publishToast } = useToast()

  const getServices = async () => {
    try {
      const response = await fetch('/api/v1/json/service/get', { credentials: 'include' })
      let apiResponse: APIResponse<Record<Services, ServiceInfo>> = await response.json()

      if (apiResponse.success) {
        services.value = apiResponse.data
      } else {
        publishToast('Unable to get services', 'Error: ' + apiResponse.error_msg, 10, true)
      }
    } catch (err) {
      publishToast('Unable to get services', 'Error: ' + (err as any).toString(), 10, true)
    }
  }

  const testService = async (serviceInfo: ServiceInfo): Promise<APIResponse<ServiceStatus>> => {
    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(serviceInfo),
      credentials: 'include',
    }

    const apiEndpoint = '/api/v1/json/service/status'

    try {
      const response = await fetch(apiEndpoint, requestOptions)
      let apiResponse: APIResponse<ServiceStatus> = await response.json()
      if (!apiResponse.success) publishToast('Unable to test service', 'Error: ' + apiResponse.error_msg, 10, true)
      return apiResponse
    } catch (err: any) {
      publishToast('Unable to test service', 'Error: ' + (err as any).toString(), 10, true)
      const apiResponse: APIResponse<ServiceStatus> = {
        success: false,
        error_msg: (err as any).toString(),
      }
      return apiResponse
    }
  }

  const saveService = async (serviceInfo: ServiceInfo): Promise<APIResponse<ServiceStatus>> => {
    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(serviceInfo),
      credentials: 'include',
    }

    const apiEndpoint = '/api/v1/json/service/save'

    try {
      const response = await fetch(apiEndpoint, requestOptions)
      let apiResponse: APIResponse<ServiceStatus> = await response.json()
      if (!apiResponse.success) publishToast('Unable to save service', 'Error: ' + apiResponse.error_msg, 10, true)
      return apiResponse
    } catch (err: any) {
      publishToast('Unable to test service', 'Error: ' + (err as any).toString(), 10, true)
      const apiResponse: APIResponse<ServiceStatus> = {
        success: false,
        error_msg: err,
      }
      return apiResponse
    }
  }

  return { saveService, testService, getServices, services }
})
