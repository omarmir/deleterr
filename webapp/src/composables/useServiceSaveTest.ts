import { ref } from 'vue'
import { ServiceInfo, APIResponse, TestState } from '~/@types/deleterr'

export function useServiceSaveTest() {
  enum ServiceOperations {
    Save = 'save',
    Test = 'test',
  }

  const operationState = ref({ test: TestState.hidden, save: TestState.hidden })
  const errorMsg = ref('')

  const saveTestService = async (serviceInfo: ServiceInfo, operation: ServiceOperations) => {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(serviceInfo),
    }

    const apiEndpoint =
      operation === ServiceOperations.Save
        ? 'http://localhost:8080/api/v1/json/service/save'
        : 'http://localhost:8080/api/v1/json/service/status'

    try {
      operationState.value[operation] = TestState.loading
      const response = await fetch(apiEndpoint, requestOptions)
      let apiResponse: APIResponse<ServiceInfo> = await response.json()

      if (apiResponse.success) {
        operationState.value[operation] = TestState.success
        setTimeout(() => {
          operationState.value[operation] = TestState.hidden
        }, 5000)
      } else {
        operationState.value[operation] = TestState.failure
        errorMsg.value = apiResponse.error_msg ?? ''
      }
    } catch (error: any) {
      console.error(error)
      operationState.value[operation] = TestState.failure
      errorMsg.value = error
    }
  }

  return {
    saveTestService,
    operationState,
    errorMsg,
    ServiceOperations,
  }
}
