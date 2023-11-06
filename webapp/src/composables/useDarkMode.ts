import { ref } from 'vue'
import type { Ref } from 'vue'

const lsIsDark: boolean = localStorage.getItem('isDark') === 'true'
const isDark: Ref<boolean> = ref(lsIsDark)

export function useDarkMode() {
  const toggleDarkMode = () => {
    isDark.value = !isDark.value
    localStorage.setItem('isDark', isDark.value.toString())
  }

  const isDarkModeEnabled: Ref<boolean> = isDark

  return {
    toggleDarkMode,
    isDarkModeEnabled,
  }
}
