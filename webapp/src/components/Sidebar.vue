<template>
  <aside
    v-on-click-outside="outsideClick"
    class="z-20 h-full w-64 flex-shrink-0 overflow-y-auto bg-white dark:bg-gray-800 md:block"
    :class="isSideMenuOpenNow ? 'fixed pt-4' : 'hidden'">
    <div v-if="isSideMenuOpenNow" class="flex h-8 flex-row items-center px-5">
      <button
        ref="ignoreElRef"
        class="rounded-md p-1 text-purple-600 focus:shadow-outline-purple focus:outline-none dark:text-purple-300 md:hidden"
        aria-label="Menu"
        @click="toggleSideMenu">
        <svg class="h-6 w-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
            clip-rule="evenodd"></path>
        </svg>
      </button>
    </div>
    <div class="py-4 text-gray-500 dark:text-gray-400">
      <Logo />
      <ul class="mt-6">
        <MenuItem to="/">
          <template #svgPath>
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"></path>
          </template>
          Dashboard
        </MenuItem>
        <MenuItem to="/services">
          <template #svgPath>
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z"></path>
          </template>
          Services
        </MenuItem>
        <MenuItem to="/settings">
          <template #svgPath>
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M21.75 6.75a4.5 4.5 0 01-4.884 4.484c-1.076-.091-2.264.071-2.95.904l-7.152 8.684a2.548 2.548 0 11-3.586-3.586l8.684-7.152c.833-.686.995-1.874.904-2.95a4.5 4.5 0 016.336-4.486l-3.276 3.276a3.004 3.004 0 002.25 2.25l3.276-3.276c.256.565.398 1.192.398 1.852z"></path>
            <path stroke-linecap="round" stroke-linejoin="round" d="M4.867 19.125h.008v.008h-.008v-.008z"></path>
          </template>
          Settings
        </MenuItem>
        <MenuItem to="/schedule">
          <template #svgPath>
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </template>
          Schedule
        </MenuItem>
      </ul>
    </div>
  </aside>
</template>
<script lang="ts" setup>
import Logo from '~/components/Logo.vue'
import MenuItem from '~/components/Sidebar/MenuItem.vue'
import { useSideMenu } from '~/composables/useSideMenu'
import { vOnClickOutside } from '@vueuse/components'

const { isSideMenuOpenNow, outsideClick, toggleSideMenu } = useSideMenu()
</script>
