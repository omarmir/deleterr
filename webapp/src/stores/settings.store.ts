import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { APIResponse, AuthenticationUser } from '~/@types/deleterr'
import { Settings } from '~/@types/settings'

export const useSettingsStore = defineStore('settings', () => {
  const newPassword = ref(undefined)
  return {
    newPassword,
  }
})
