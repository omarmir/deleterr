import { defineStore } from 'pinia'
import { ref } from 'vue'
import { APIResponse, ServiceInfo, Services } from '~/@types/deleterr'
import { ServiceStatus } from '~/@types/services'

export const useServiceStore = defineStore('services', () => {
  const services = ref<Record<Services, ServiceInfo> | undefined>(undefined)

  const getServices = async () => {
    try {
      const response = await fetch('/api/v1/json/service/get', { credentials: 'include' })
      let apiResponse: APIResponse<Record<Services, ServiceInfo>> = await response.json()
      services.value = apiResponse.data
    } catch (error) {
      console.error(error)
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
      return apiResponse
    } catch (error: any) {
      console.error(error)
      const apiResponse: APIResponse<ServiceStatus> = {
        success: false,
        error_msg: error,
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
      return apiResponse
    } catch (error: any) {
      console.error(error)
      const apiResponse: APIResponse<ServiceStatus> = {
        success: false,
        error_msg: error,
      }

      return apiResponse
    }
  }

  return { saveService, testService, getServices, services }
})
