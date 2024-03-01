<template>
  <label class="flex items-center dark:text-gray-400">
    <div class="flex flex-row place-items-center">
      <span class="text-gray-700 dark:text-gray-400" :class="{ 'grow-1 lg:basis-1/2': isHorizontal }">
        {{ label }}
        <span v-if="required" class="text-red-600">*</span>
      </span>
      <select
        class="block appearance-none rounded-md border bg-white py-2 pl-2 pr-8 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
        @input="setValue">
        <option v-for="item in options" :key="item.name" :value="item.value">
          {{ item.name }}
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
    <span class="ml-2">
      <slot></slot>
    </span>
  </label>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'

const props = defineProps({
  label: String,
  placeholder: String,
  modelValue: Boolean,
  options: { type: Array as PropType<{ name: string; value: string }[]>, required: true },
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  required: {
    type: Boolean,
    required: true,
    default: false,
  },
  isHorizontal: {
    type: Boolean,
    required: false,
    default: false,
  },
})

const setValue = (event: Event): void => {
  const newVal: boolean = (event.target as HTMLInputElement).checked
  if (!props.disabled) emit('update:modelValue', newVal)
}

const emit = defineEmits(['update:modelValue'])
</script>
