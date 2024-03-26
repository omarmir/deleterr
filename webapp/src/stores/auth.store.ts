import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { APIResponse, AuthenticationUser } from '~/@types/deleterr'
import { useRouter } from 'vue-router'
import { useToast } from '~/composables/useToast'

export const useAuthStore = defineStore('auth', () => {
  const username: Ref<string | undefined> = ref(undefined)
  const isLoggedIn: Ref<boolean | undefined> = ref(undefined)
  const { publishToast } = useToast()

  const router = useRouter()

  async function checkUsersSetup(): Promise<boolean | undefined> {
    const checkUsersEndpoint = `/auth/setup/status`
    const requestOptions: RequestInit = {
      method: 'GET',
      headers: { 'Content-Type': 'application/json' },
    }

    try {
      const response = await fetch(checkUsersEndpoint, requestOptions)
      let apiResponse: APIResponse<boolean> = await response.json()
      return apiResponse.data
    } catch (err) {
      publishToast('Unable to confirm a user has been initialized', 'Error: ' + (err as any).toString(), 10, true)
    }
  }

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
      publishToast('Unable to validate session', 'Error: ' + (err as any).toString(), 10, true)
      isLoggedIn.value = false
      username.value = undefined
    } finally {
      return isLoggedIn.value ?? false
    }
  }

  async function login(authUser: AuthenticationUser): Promise<APIResponse<string>> {
    const loginEndpoint = `/auth/login`
    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(authUser),
      credentials: 'include',
    }

    // Simulate delay
    //await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(loginEndpoint, requestOptions)
      const apiResponse: APIResponse<string> = await response.json()
      if (apiResponse.success) {
        ;[username.value, isLoggedIn.value] = [apiResponse.data ?? '', true]
        sessionStorage.setItem('loggedUser', apiResponse.data ?? '')
      } else {
        sessionStorage.removeItem('loggedUser')
        publishToast('Unable to Login', 'Error: ' + apiResponse.error_msg, 10, true)
      }

      return apiResponse
    } catch (err: any) {
      ;[username.value, isLoggedIn.value] = [undefined, false]
      sessionStorage.removeItem('loggedUser')
      const apiResponse: APIResponse<string> = {
        success: false,
        error_msg: err,
      }
      publishToast('Unable to login', 'Error: ' + (err as any).toString(), 10, true)
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
        publishToast('Unable to Login', 'Error: ' + apiResponse.error_msg, 10, true)
      }
    } catch (err) {
      publishToast('Unable to logout', 'Error: ' + (err as any).toString(), 10, true)
      console.log((err as any).toString())
    }
  }

  async function addInitialUser(authUser: AuthenticationUser): Promise<APIResponse<boolean | undefined>> {
    const checkUsersEndpoint = `/auth/user/initialize`
    const requestOptions: RequestInit = {
      method: 'POST',
      body: JSON.stringify(authUser),
      headers: { 'Content-Type': 'application/json' },
    }

    try {
      const response = await fetch(checkUsersEndpoint, requestOptions)
      let apiResponse: APIResponse<boolean> = await response.json()
      if (apiResponse.success) {
        publishToast('User added', 'Login with credentials.', 10, false)
      } else {
        publishToast('Unable to add user', apiResponse.error_msg ?? '', 10, true)
      }

      return apiResponse
    } catch (err: any) {
      const apiResponse: APIResponse<undefined> = {
        success: false,
        error_msg: err,
      }
      publishToast('Unable to add user', 'Error: ' + apiResponse.error_msg, 10, true)
      return apiResponse
    }
  }
  return { validateSession, username, login, logout, isLoggedIn, checkUsersSetup, addInitialUser }
})
