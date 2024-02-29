import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { APIResponse, AuthenticationUser } from '~/@types/deleterr'
import { useRouter } from 'vue-router'

export const useAuthStore = defineStore('auth', () => {
  const username: Ref<string | undefined> = ref(undefined)
  const isLoggedIn: Ref<boolean | undefined> = ref(undefined)

  const router = useRouter()

  async function validateSession(): Promise<boolean> {
    if (!sessionStorage.getItem('loggedUser')) return false

    if (isLoggedIn.value) return isLoggedIn.value

    const validateEndpoint = `/api/v1/json/auth/user/validate`
    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(sessionStorage.getItem('loggedUser')),
      credentials: 'include',
    }

    try {
      const response = await fetch(validateEndpoint, requestOptions)
      let apiResponse: APIResponse<null> = await response.json()
      isLoggedIn.value = apiResponse.success
      username.value = sessionStorage.getItem('loggedUser') ?? undefined
    } catch (err) {
      console.log((err as any).toString())
      isLoggedIn.value = false
      username.value = undefined
    } finally {
      return isLoggedIn.value ?? false
    }
  }

  const login = async function login(authUser: AuthenticationUser): Promise<APIResponse<string>> {
    const loginEndpoint = `/auth/login`
    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(authUser),
      credentials: 'include',
    }

    // Simulate delay
    await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(loginEndpoint, requestOptions)
      const apiResponse: APIResponse<string> = await response.json()
      if (apiResponse.success) {
        ;[username.value, isLoggedIn.value] = [apiResponse.data ?? '', true]
        sessionStorage.setItem('loggedUser', apiResponse.data ?? '')
        //router.push(originalPath.value ?? '/')
        //originalPath.value = undefined
      } else {
        console.log(apiResponse.error_msg)
        sessionStorage.removeItem('loggedUser')
      }

      return apiResponse

    } catch (err: any) {
      ;[username.value, isLoggedIn.value] = [undefined, false]
      sessionStorage.removeItem('loggedUser')
      const apiResponse: APIResponse<string> = {
        success: false,
        error_msg: err
      }

      return apiResponse
    }
  }

  async function logout() {
    const logoutEndpoint = `/auth/logout`
    const requestOptions: RequestInit = {
      method: 'POST',
      credentials: 'include',
    }

    try {
      const response = await fetch(logoutEndpoint, requestOptions)
      let apiResponse: APIResponse<string> = await response.json()
      if (apiResponse.success) {
        ;[username.value, isLoggedIn.value] = [undefined, false]
        sessionStorage.removeItem('loggedUser')
        router.push('/login')
      } else {
        console.log(apiResponse.error_msg)
      }
    } catch (err) {
      console.log((err as any).toString())
    }
  }

  return { validateSession, username, login, logout, isLoggedIn }
})
