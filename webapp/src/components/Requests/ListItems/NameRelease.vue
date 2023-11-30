<template>
  <div class="flex flex-col space-y-1 text-gray-700 dark:text-gray-400">
    <p class="font-semibold">{{ title ?? 'N/A' }}</p>
    <p class="text-xs">Release: {{ releaseDate ?? 'N/A' }}</p>
  </div>
</template>
<script setup lang="ts">
defineProps({
  title: { type: String, required: false },
  releaseDate: { type: String, required: false },
})
</script>
