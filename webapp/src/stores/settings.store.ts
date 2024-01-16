import { defineStore } from 'pinia'
import { ref, Ref } from 'vue'
import { SettingNames, Settings } from '~/@types/settings'
//import { APIResponse, AuthenticationUser } from '~/@types/deleterr'

export const useSettingsStore = defineStore('settings', () => {
  const newPassword: Ref<string | undefined> = ref(undefined)
  const settings: Ref<Record<string, string | boolean>> = ref({})

  const setSetting = (settingTuple: [string, string | boolean]) => settings.value[settingTuple[0]] = settingTuple[1]

  return {
    newPassword,
    settings,
    setSetting
  }
})
