import { ref } from 'vue'
import { ServiceInfo, APIResponse, TestState } from '~/@types/deleterr'

export function useServiceTest() {
  const testState = ref(TestState.hidden)
  const errorMsg = ref('')

  const testService = async (serviceInfo: ServiceInfo) => {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(serviceInfo),
    }

    try {
      testState.value = TestState.loading
      const response = await fetch('http://localhost:8080/api/v1/json/service/status', requestOptions)
      let apiResponse: APIResponse<ServiceInfo> = await response.json()

      if (apiResponse.success) {
        testState.value = TestState.success
        setTimeout(() => {
          testState.value = TestState.hidden
        }, 5000)
      } else {
        testState.value = TestState.failure
        errorMsg.value = apiResponse.error_msg ?? ''
      }
    } catch (error: any) {
      console.error(error)
      testState.value = TestState.failure
      errorMsg.value = error
    }
  }

  return {
    testService,
    testState,
    errorMsg,
  }
}
