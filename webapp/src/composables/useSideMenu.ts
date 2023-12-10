import { ref } from 'vue'
import type { Ref } from 'vue'

const isSideMenuOpen: Ref<boolean> = ref(false)

export function useSideMenu() {
  const toggleSideMenu = () => {
    isSideMenuOpen.value = !isSideMenuOpen.value
  }

  const outsideClick = () => {
    if (isSideMenuOpen.value) {
      isSideMenuOpen.value = false
      isSideMenuOpenNow.value = isSideMenuOpen.value
    }
  }

  const isSideMenuOpenNow: Ref<boolean> = isSideMenuOpen

  return {
    toggleSideMenu,
    isSideMenuOpenNow,
    outsideClick,
  }
}
