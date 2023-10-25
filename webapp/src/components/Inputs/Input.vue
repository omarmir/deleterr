<template>
  <label class="flex flex-col space-y-3 text-sm">
    <span class="text-gray-700 dark:text-gray-400">
      {{ label }}
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
  label: String,
  placeholder: String,
  modelValue: String,
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
  const searchVal: string = (event.target as HTMLInputElement).value
  if (!props.disabled) emit('update:modelValue', searchVal)
}

const emit = defineEmits(['update:modelValue'])
</script>
