import { ServiceInfo, APIResponse } from '~/@types/deleterr'

export function useServiceSave() {
  const saveService = async (serviceInfo: ServiceInfo) => {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(serviceInfo),
    }

    try {
      const response = await fetch('http://localhost:8080/api/v1/json/service/save', requestOptions)
      let apiResponse: APIResponse<String> = await response.json()
      console.log(apiResponse)
    } catch (error) {
      console.error(error)
    }
  }

  return {
    saveService,
  }
}
