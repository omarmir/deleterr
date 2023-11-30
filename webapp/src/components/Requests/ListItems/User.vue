<template>
  <div class="flex items-center text-sm text-gray-700 dark:text-gray-400">
    <div class="relative mr-3 flex flex-row place-items-center space-x-2 rounded-full">
      <img class="h-5 w-5 rounded-full object-cover" :src="mediaRequest?.requestedBy.avatar" alt="" loading="lazy" />
      <p class="font-semibold">
        {{ mediaRequest?.requestedBy.plexUsername ?? mediaRequest?.requestedBy.username }}
      </p>
      <div class="h-2 w-2 rounded-full" :data-user-type="mediaRequest?.requestedBy.userType"></div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { PropType } from 'vue'
import { MediaRequest } from '~/@types/deleterr.ts'
defineProps({
  mediaRequest: { type: Object as PropType<MediaRequest>, required: false },
})
</script>
<style lang="postcss" scoped>
[data-user-type='1'] {
  @apply bg-purple-600;
}

[data-user-type='2'] {
  @apply bg-teal-600;
}
</style>
