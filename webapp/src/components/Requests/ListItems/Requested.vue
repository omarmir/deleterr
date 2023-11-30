<template>
  <div class="text-gray-700 dark:text-gray-400">
    <p class="text-sm">{{ getDate(createdAt) }}</p>
    <p class="text-xs">{{ getTime(createdAt) }}</p>
  </div>
</template>
<script setup lang="ts">
defineProps({
  createdAt: { type: String, required: false },
})

// YYYY-MM-DD - dont like it? Make your own app. Just be happy its not my preferred 24H
const getDate = (newDate?: string) => (newDate ? new Date(newDate).toLocaleDateString('en-CA') : 'N/A')
const getTime = (newDate?: string) => (newDate ? new Date(newDate).toLocaleTimeString('en-CA') : 'N/A')
</script>
