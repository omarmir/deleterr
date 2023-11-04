import { ref } from 'vue'
import { ServiceInfo, APIResponse, ButtonState } from '~/@types/deleterr'

export function useServiceTest() {
  const testState = ref(ButtonState.hidden)

  const testService = async (serviceInfo: ServiceInfo) => {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(serviceInfo),
    }

    try {
      testState.value = ButtonState.loading
      const response = await fetch('http://localhost:8080/api/v1/json/service/status', requestOptions)
      let apiResponse: APIResponse<ServiceInfo> = await response.json()

      if (apiResponse.success) {
        testState.value = ButtonState.success
        setTimeout(() => {
          testState.value = ButtonState.hidden
        }, 5000)
      } else {
        testState.value = ButtonState.failure
      }
    } catch (error: any) {
      console.error(error)
      testState.value = ButtonState.failure
    }
  }

  return {
    testService,
    testState,
  }
}
