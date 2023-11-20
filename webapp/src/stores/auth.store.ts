import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { APIResponse, AuthenticationUser } from '~/@types/deleterr'
import { useRouter } from 'vue-router'

export const useAuthStore = defineStore('auth', () => {
  const username: Ref<string | undefined> = ref(undefined)
  const isLoggedIn: Ref<boolean> = ref(false)
  const originalPath: Ref<string | undefined> = ref(undefined)
  const router = useRouter()

  const login = async function login(authUser: AuthenticationUser) {
    const loginEndpoint = `/auth/login`
    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(authUser),
      credentials: 'include',
    }

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(loginEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()
      if (apiResponse.success) {
        [username.value, isLoggedIn.value] = [apiResponse.data ?? '', true]
        router.push(originalPath.value ?? '/')
        originalPath.value = undefined
      } else {
        console.log(apiResponse.error_msg)
      }
    } catch (err) {
      console.log((err as any).toString())
    }
  }

  async function logout() {
    [username.value, isLoggedIn.value] = [undefined, false]
  }

  return { isLoggedIn, username, login, logout, originalPath }
})
