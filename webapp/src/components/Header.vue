<!-- eslint-disable vue/no-v-html -->
<template>
  <header class="z-10 bg-white py-4 shadow-md dark:bg-gray-800">
    <div class="flex h-full items-center justify-between px-6 text-purple-600 dark:text-purple-300">
      <!-- Mobile hamburger -->
      <button
        ref="ignoreElRef"
        class="-ml-1 mr-5 rounded-md p-1 focus:shadow-outline-purple focus:outline-none md:hidden"
        aria-label="Menu"
        @click="toggleSideMenu">
        <svg class="h-6 w-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
            clip-rule="evenodd"></path>
        </svg>
      </button>
      <!-- Search input -->
      <div class="flex flex-1 justify-center lg:mr-32">
        <div class="relative mr-6 w-full max-w-xl focus-within:text-purple-500">
          <div class="absolute inset-y-0 flex items-center pl-2">
            <svg class="h-4 w-4" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20">
              <path
                fill-rule="evenodd"
                d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                clip-rule="evenodd"></path>
            </svg>
          </div>
          <input
            class="form-input h-8 w-full rounded-md border-0 bg-gray-100 pl-8 pr-2 text-sm text-gray-700 placeholder-gray-600 focus:border-purple-300 focus:bg-white focus:placeholder-gray-500 focus:shadow-outline-purple focus:outline-none focus:ring-0 dark:bg-gray-700 dark:text-gray-200 dark:placeholder-gray-500 dark:focus:placeholder-gray-600 dark:focus:shadow-outline-gray"
            type="text"
            placeholder="Search for requests"
            aria-label="Search"
            @input="requestStore.search" />
        </div>
      </div>
      <ul class="flex flex-shrink-0 items-center space-x-6">
        <!-- Theme toggler -->
        <li class="flex">
          <button
            class="rounded-md focus:shadow-outline-purple focus:outline-none"
            aria-label="Toggle color mode"
            @click="toggleDarkMode">
            <template v-if="!isDarkModeEnabled">
              <svg class="h-6 w-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20">
                <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
              </svg>
            </template>
            <template v-if="isDarkModeEnabled">
              <svg class="h-6 w-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20">
                <path
                  fill-rule="evenodd"
                  d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"
                  clip-rule="evenodd"></path>
              </svg>
            </template>
          </button>
        </li>
        <!-- Profile menu -->
        <li class="relative">
          <button
            class="rounded-full align-middle focus:shadow-outline-purple focus:outline-none"
            aria-label="Profile and logout"
            aria-haspopup="true"
            @click="toggleProfileMenu">
            <div>
              <!-- icon here -->
              <div v-if="authStore.isLoggedIn">
                <div
                  class="dark: relative h-6 w-6 rounded-full bg-purple-600 p-[1px] align-middle text-white dark:bg-purple-300 dark:text-purple-600">
                  <span v-if="isDarkModeEnabled" v-html="minidenticon(authStore.username ?? '', 20, 20)"></span>
                  <span v-else v-html="minidenticon(authStore.username ?? '', 20, 100)"></span>
                </div>
              </div>
            </div>
          </button>
          <template v-if="isProfileMenuOpen">
            <ul
              v-on-click-outside="toggleProfileMenu"
              class="absolute right-0 mt-2 w-56 space-y-2 rounded-md border border-gray-100 bg-white p-2 text-gray-600 shadow-md dark:border-gray-700 dark:bg-gray-700 dark:text-gray-300"
              aria-label="submenu">
              <li class="flex">
                <a
                  class="inline-flex w-full items-center rounded-md px-2 py-1 text-sm font-semibold transition-colors duration-150 hover:bg-gray-100 hover:text-gray-800 dark:hover:bg-gray-800 dark:hover:text-gray-200"
                  href="#">
                  <svg
                    class="mr-3 h-4 w-4"
                    aria-hidden="true"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                  </svg>
                  <span>Profile</span>
                </a>
              </li>
              <li class="flex">
                <a
                  class="inline-flex w-full items-center rounded-md px-2 py-1 text-sm font-semibold transition-colors duration-150 hover:bg-gray-100 hover:text-gray-800 dark:hover:bg-gray-800 dark:hover:text-gray-200"
                  href="#">
                  <svg
                    class="mr-3 h-4 w-4"
                    aria-hidden="true"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path
                      d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                    <path d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                  </svg>
                  <span>Settings</span>
                </a>
              </li>
              <li class="flex">
                <a
                  class="inline-flex w-full cursor-pointer items-center rounded-md px-2 py-1 text-sm font-semibold transition-colors duration-150 hover:bg-gray-100 hover:text-gray-800 dark:hover:bg-gray-800 dark:hover:text-gray-200"
                  @click="authStore.logout">
                  <svg
                    class="mr-3 h-4 w-4"
                    aria-hidden="true"
                    fill="none"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path
                      d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"></path>
                  </svg>
                  <span>Log out</span>
                </a>
              </li>
            </ul>
          </template>
        </li>
      </ul>
    </div>
  </header>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import type { Ref } from 'vue'
import { useDarkMode } from '~/composables/useDarkMode'
import { useSideMenu } from '~/composables/useSideMenu'
import { useRequestsStore } from '~/stores/requests.store'
import { useAuthStore } from '~/stores/auth.store'
import { vOnClickOutside } from '@vueuse/components'
import { minidenticon } from 'minidenticons'

const requestStore = useRequestsStore()
const authStore = useAuthStore()

const { toggleDarkMode, isDarkModeEnabled } = useDarkMode()
const { toggleSideMenu } = useSideMenu()

let isProfileMenuOpen: Ref<Boolean> = ref(false)

const toggleProfileMenu = () => {
  isProfileMenuOpen.value = !isProfileMenuOpen.value
}
</script>
