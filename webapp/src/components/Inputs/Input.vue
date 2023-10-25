<template>
  <label class="flex flex-col space-y-3 text-sm">
    <span class="text-gray-700 dark:text-gray-400">
      {{ label }}
      <span v-if="required" class="text-red-600">*</span>
    </span>
    <input
      class="form-input mt-1 block w-full rounded-md border p-2 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
      :placeholder="placeholder"
      :type="type"
      :value="modelValue"
      @input="setValue" />
  </label>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'
import { InputType } from '~/@types/deleterr.ts'

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
    default: InputType.text,
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
