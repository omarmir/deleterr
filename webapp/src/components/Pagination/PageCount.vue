<template>
  <span class="col-span-3 flex items-center">
    Showing {{ take * selectedPage + 1 }}-{{ Math.min(take * selectedPage + 5, filteredRequests) }} of
    {{ filteredRequests }}
  </span>
</template>
<script lang="ts" setup>
defineProps({
  take: { type: Number, required: true },
  selectedPage: { type: Number, required: true },
  filteredRequests: { type: Number, required: true },
})
</script>
