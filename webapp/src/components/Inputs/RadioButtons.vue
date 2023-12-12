<template>
  <div class="mt-4 flex flex-col gap-4 text-sm lg:flex-row">
    <div class="grow-1 basis-1/2 text-gray-700 dark:text-gray-400">{{ label }}</div>
    <div>
      <label
        v-for="option in options"
        :key="option.name"
        class="mr-6 inline-flex items-center text-gray-600 dark:text-gray-400">
        <input
          type="radio"
          class="form-radio text-purple-600 focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:focus:shadow-outline-gray"
          name="option.name"
          :value="option.value"
          @input="setValue" />
        <span class="ml-2">{{ option.label }}</span>
      </label>
    </div>
  </div>
</template>
<script setup lang="ts">
import { SettingsOption } from '~/@types/settings'

const props = defineProps({
  options: { type: Array<SettingsOption>, required: true },
  label: {
    type: String,
    required: true,
  },
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
})

const setValue = (event: Event): void => {
  const newVal: string = (event.target as HTMLInputElement).value
  if (!props.disabled) emit('update:modelValue', newVal)
}

const emit = defineEmits(['update:modelValue'])
</script>
