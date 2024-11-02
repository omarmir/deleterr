<template>
  <div
    class="text-outlinetext fixed bottom-0 start-1 z-[55] flex w-full flex-col justify-end px-4 pt-6 sm:w-96 sm:px-6">
    <ToastBody v-for="toast in currentToasts" :key="toast.id" :toast="toast" />
  </div>
</template>
<script lang="ts" setup>
import { useToast } from '~/composables/useToast'
import ToastBody from '~/components/Toast/Body.vue'

const { currentToasts } = useToast()
</script>
