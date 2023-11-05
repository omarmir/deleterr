<template>
  <th class="cursor-pointer px-4 py-3" alt="Sort" aria-sort="none" @click="$emit('sortClicked')">
    <div class="flex gap-2">
      <slot></slot>
      <div v-if="sortIcon" class="h-4 w-4 align-middle" :class="{ 'rotate-180': isDescending }">
        <svg fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 15.75l7.5-7.5 7.5 7.5"></path>
        </svg>
      </div>
    </div>
  </th>
</template>
<script lang="ts" setup>
defineProps({
  sortIcon: { type: Boolean, required: false, default: true },
  isDescending: { type: Boolean, required: false, default: false },
})

defineEmits(['sortClicked'])
</script>
