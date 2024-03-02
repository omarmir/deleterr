<template>
  <div class="grid grid-cols-1 gap-2 lg:grid-cols-2">
    <label class="mr-2 flex place-items-center justify-start text-sm" :for="name">
      <span class="text-gray-700 dark:text-gray-400">
        {{ label }}
        <span v-if="required" class="text-red-600">*</span>
      </span>
    </label>
    <div class="flex grow flex-row place-items-center">
      <slot></slot>
    </div>
    <div class="flex grow flex-row text-sm text-gray-600 dark:text-gray-500">
      <slot name="subtitle"></slot>
    </div>
  </div>
</template>
<script lang="ts" setup>
defineProps({
  label: {
    type: String,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
  required: {
    type: Boolean,
    required: false,
    default: false,
  },
})
</script>
