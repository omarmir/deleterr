<template>
  <div class="flex flex-row place-items-center gap-2 text-xs text-gray-700 dark:text-gray-400">
    <p class="text-sm font-bold">Seasons</p>
    <div class="flex flex-row gap-1">
      <div
        v-for="season in seasonStatus"
        :key="season.seasonNumber"
        :data-status="season.watched.toLowerCase()"
        class="flex h-5 w-5 place-content-center items-center rounded-full">
        {{ season.seasonNumber }}
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { PropType } from 'vue'
import { MediaType, SeasonStatus } from '~/@types/deleterr.ts'
defineProps({
  mediaType: { type: String as PropType<MediaType>, required: false },
  seasonStatus: { type: Array<SeasonStatus>, required: false },
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
