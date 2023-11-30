<template>
  <div>
    <span
      :data-status="watchedStatus.toLowerCase()"
      class="rounded-full px-2 py-1 font-semibold capitalize leading-tight">
      {{
        //@ts-ignore - enums in TS are weird
        WatchedStatus[watchedStatus]
      }}
    </span>
  </div>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'
import { WatchedStatus } from '~/@types/deleterr'

defineProps({
  watchedStatus: { type: String as PropType<WatchedStatus>, required: false, default: WatchedStatus.Unwatched },
})
</script>
<style lang="postcss" scoped>
[data-status='watched'] {
  @apply bg-green-100 text-green-700 dark:bg-green-700 dark:text-green-100;
}
[data-status='inprogress'] {
  @apply bg-orange-100 text-orange-700 dark:bg-orange-600 dark:text-white;
}
[data-status='unwatched'] {
  @apply bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-100;
}
[data-status='unknown'] {
  @apply bg-red-100 text-red-700 dark:bg-red-700 dark:text-red-100;
}
</style>
