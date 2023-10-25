import { ServiceInfo, APIResponse } from '~/@types/deleterr'

export function useServiceTest() {
  const testService = async (serviceInfo: ServiceInfo) => {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(serviceInfo),
    }

    try {
      const response = await fetch('http://localhost:8080/api/v1/json/service/status', requestOptions)
      let apiResponse: APIResponse<ServiceInfo> = await response.json()
      console.log(apiResponse)
    } catch (error) {
      console.error(error)
    }
  }

  return {
    testService,
  }
}
