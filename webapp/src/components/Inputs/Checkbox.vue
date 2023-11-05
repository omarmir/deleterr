<template>
  <label class="flex items-center dark:text-gray-400">
    <input
      :checked="modelValue"
      type="checkbox"
      :placeholder="placeholder"
      :value="modelValue"
      class="form-checkbox text-purple-600 focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:focus:shadow-outline-gray"
      @input="setValue" />
    <span class="ml-2">
      <slot></slot>
    </span>
  </label>
</template>
<script lang="ts" setup>
const props = defineProps({
  label: String,
  placeholder: String,
  modelValue: Boolean,
  disabled: {
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
