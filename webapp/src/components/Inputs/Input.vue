<template>
  <label class="flex flex-col gap-2 text-sm" :class="{ 'lg:flex-row lg:place-items-center': isHorizontal }">
    <span class="text-gray-700 dark:text-gray-400" :class="{ 'grow-1 lg:basis-1/2': isHorizontal }">
      {{ label }}
      <span v-if="required" class="text-red-600">*</span>
    </span>
    <input
      :class="isHorizontal ? '' : 'mt-1 w-full'"
      class="form-input block rounded-md border p-2 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
      :placeholder="placeholder"
      :type="type"
      :value="modelValue"
      :label="label"
      @input="setValue" />
  </label>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'
import { InputType } from '~/@types/common'

const props = defineProps({
  label: {
    type: String,
    required: true,
  },
  placeholder: String,
  modelValue: String,
  required: {
    type: Boolean,
    required: false,
    default: false,
  },
  type: {
    type: String as PropType<InputType>,
    required: false,
    default: 'text',
  },
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
  isHorizontal: {
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
