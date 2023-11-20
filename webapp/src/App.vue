<template>
  <div :class="{ dark: isDarkModeEnabled }" class="font-inter">
    <router-view></router-view>
    <ToastWrapper />
  </div>
</template>

<script lang="ts" setup>
import { useDarkMode } from '~/composables/useDarkMode'
import ToastWrapper from '~/components/Toast/Wrapper.vue'
const { isDarkModeEnabled } = useDarkMode()
</script>

<style scoped></style>
