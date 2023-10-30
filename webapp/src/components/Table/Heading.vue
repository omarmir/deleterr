<template>
  <th class="cursor-pointer px-4 py-3" alt="Sort" aria-sort="none">
    <div class="flex gap-2">
      <slot></slot>
      <div v-if="sortIcon" class="h-4 w-4 align-middle">
        <svg
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
          aria-hidden="false">
          <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5"></path>
        </svg>
      </div>
    </div>
  </th>
</template>
<script lang="ts" setup>
defineProps({
  sortIcon: { type: Boolean, required: false, default: true },
})
</script>
