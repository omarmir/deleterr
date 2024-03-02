<template>
  <div class="flex w-full flex-row place-items-center">
    <select
      v-model="model"
      class="block w-full appearance-none rounded-md border bg-white py-2 pl-2 pr-8 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
      :required="required"
      :name="name">
      <option v-for="item in options" :key="item.value" :value="item.value">
        {{ item.label }}
      </option>
    </select>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      strokeWidth="{1.5}"
      stroke="currentColor"
      class="pointer-events-none -ml-6 h-4 w-4 dark:text-gray-300">
      <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
    </svg>
  </div>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'

defineProps({
  name: {
    type: String,
    required: true,
  },
  options: { type: Array as PropType<{ value: string; label: string }[]>, required: true },
  required: {
    type: Boolean,
    required: true,
    default: false,
  },
})

const model = defineModel({ type: String })
</script>
