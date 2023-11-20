import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { APIResponse, AuthenticationUser } from '~/@types/deleterr'

export const useAuthStore = defineStore('auth', () => {
  const username: Ref<string> = ref('')
  const isLoggedIn: Ref<boolean> = ref(false)

  async function login(authUser: AuthenticationUser) {
    const loginEndpoint = `http://localhost:8080/auth/login`
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(authUser),
    }

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(loginEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()
      if (apiResponse.success) {
        username.value = apiResponse.data ?? ''
      } else {
        console.log(apiResponse.error_msg)
      }
    } catch (err) {
      console.log((err as any).toString())
    }
  }

  return { isLoggedIn, username, login }
})
