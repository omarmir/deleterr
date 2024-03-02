<template>
  <input
    class="w-full appearance-none rounded-md border bg-white py-2 pl-2 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
    :placeholder="placeholder"
    :type="type"
    :value="modelValue"
    :name="name"
    :required="required"
    @input="setValue" />
</template>
<script lang="ts" setup>
import { PropType } from 'vue'

defineProps({
  placeholder: String,
  modelValue: String,
  required: {
    type: Boolean,
    required: false,
    default: false,
  },
  type: {
    type: String as PropType<'number' | 'text'>,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
})

const setValue = (event: Event): void => {
  const newVal: string = (event.target as HTMLInputElement).value
  //if (!props.disabled)
  emit('update:modelValue', newVal)
}

const emit = defineEmits(['update:modelValue'])
</script>
