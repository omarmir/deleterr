import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { APIResponse, Settings } from '~/@types/deleterr'
import { useToast } from '~/composables/useToast'

export const useSettingsStore = defineStore('settings', () => {
  const { publishToast } = useToast()

  const newPassword: Ref<{ newPassword: string | undefined }> = ref({ newPassword: undefined }) // nested nonsense to accomodate vuelidate

  const settings: Ref<Settings> = ref({
    tvPurgeMarker: 'watched',
    tvWatchedMarker: 'watched',
    tvPurgeDelay: undefined,
    tvPurgeStrategy: 'season',
    moviePurgeMarker: 'watched',
    moviePurgeDelay: undefined,
  })

  const updatePassword = async (): Promise<APIResponse<boolean> | undefined> => {
    const saveSettingsEndpoint = '/api/v1/json/auth/user/password'

    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(newPassword.value.newPassword),
      credentials: 'include',
    }

    try {
      const response = await fetch(saveSettingsEndpoint, requestOptions)
      let apiResponse: APIResponse<boolean> = await response.json()

      if (apiResponse.success) {
        publishToast('Updated', 'Password updated.', 3, false)
        newPassword.value.newPassword = undefined
        return apiResponse
      } else {
        publishToast('Unable update password', 'Error: ' + apiResponse.error_msg ?? 'Unknown!', 3, true)
        return apiResponse
      }
    } catch (err: any) {
      const apiResponse: APIResponse<boolean> = {
        success: false,
        data: false,
        error_msg: err,
      }
      publishToast('Unable update password', 'Error: ' + err.toString(), 10, true)
      return apiResponse
    }

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))
  }

  const getSettings = async () => {
    try {
      const response = await fetch('/api/v1/json/settings/get', { credentials: 'include' })
      let apiResponse: APIResponse<Settings> = await response.json()

      if (apiResponse.success) {
        settings.value = apiResponse.data ?? settings.value
      } else {
        publishToast('Unable to get settings', 'Error: ' + apiResponse.error_msg, 10, true)
      }
    } catch (err) {
      publishToast('Unable to get settings', 'Error: ' + (err as any).toString(), 10, true)
    }
  }

  const saveSettings = async (): Promise<APIResponse<Settings> | undefined> => {
    const saveSettingsEndpoint = '/api/v1/json/settings/save'

    const requestOptions: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(settings.value),
      credentials: 'include',
    }

    // Simulate delay
    // await new Promise((resolve) => setTimeout(resolve, 2000))

    try {
      const response = await fetch(saveSettingsEndpoint, requestOptions)
      let apiResponse: APIResponse<Settings> = await response.json()

      if (apiResponse.success) {
        publishToast('Saved', 'Settings have been saved. Try a dry run to confirm.', 3, false)
        return apiResponse
      } else {
        publishToast('Unable save settings', 'Error: ' + apiResponse.error_msg ?? 'Unknown!', 3, true)
        return apiResponse
      }
    } catch (err: any) {
      const apiResponse: APIResponse<Settings> = {
        success: false,
        error_msg: err,
      }
      publishToast('Unable save settings', 'Error: ' + err.toString(), 10, true)
      return apiResponse
    }
  }

  return {
    newPassword,
    settings,
    saveSettings,
    getSettings,
    updatePassword,
  }
})
