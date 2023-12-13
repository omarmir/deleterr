<template>
  <div class="flex flex-col space-y-1 text-gray-700 dark:text-gray-400">
    <p class="text-sm font-semibold">{{ title ?? 'N/A' }}</p>
    <p class="text-xs">Release: {{ getDate(releaseDate) ?? 'N/A' }}</p>
    <p v-if="ended" class="text-xs text-red-500">Ended</p>
  </div>
</template>
<script setup lang="ts">
defineProps({
  title: { type: String, required: false },
  releaseDate: { type: String, required: false },
  ended: { type: Boolean, required: false },
})

// YYYY-MM-DD - dont like it? Make your own app. Just be happy its not my preferred 24H
const getDate = (newDate?: string) => (newDate ? new Date(newDate).toLocaleDateString('en-CA') : 'N/A')
</script>
