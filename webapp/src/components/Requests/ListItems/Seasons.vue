<template>
  <div v-if="mediaType === 'tv'" class="flex flex-row space-x-2 text-xs text-gray-700 dark:text-gray-400">
    <div v-for="season in seasonStatus" :key="season.seasonNumber">
      {{ season.seasonNumber }} - {{ season.watched }}
    </div>
  </div>
</template>
<script setup lang="ts">
import { PropType } from 'vue'
import { MediaType, SeasonStatus } from '~/@types/deleterr.ts'
defineProps({
  mediaType: { type: Object as PropType<MediaType>, required: false },
  seasonStatus: { type: Array<SeasonStatus>, required: false },
})
</script>
