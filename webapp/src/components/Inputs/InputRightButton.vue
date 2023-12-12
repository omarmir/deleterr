<template>
  <label class="block text-sm">
    <span class="text-gray-700 dark:text-gray-400">
      {{ label }}
      <span v-if="required" class="text-red-600">*</span>
    </span>
    <div class="relative mt-3 block text-gray-500 focus-within:text-purple-600">
      <input
        class="form-input mt-1 block w-full rounded-md border p-2 pr-20 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
        :placeholder="placeholder"
        :type="type"
        :value="modelValue"
        @input="setValue" />
      <ButtonsStatused
        :is-outlined="false"
        class="absolute inset-y-0 right-0 rounded-r-md border border-transparent bg-purple-600 px-4 text-sm font-medium leading-5 text-white transition-colors duration-150 hover:bg-purple-700 focus:shadow-outline-purple focus:outline-none active:bg-purple-600"
        :is-submit="true">
        {{ buttonLabel }}
      </ButtonsStatused>
    </div>
  </label>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'
import { InputType } from '~/@types/common'
import ButtonsStatused from '~/components/Buttons/Statused.vue'

const props = defineProps({
  label: {
    type: String,
    required: true,
  },
  buttonLabel: {
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
})

const setValue = (event: Event): void => {
  const newVal: string = (event.target as HTMLInputElement).value
  if (!props.disabled) emit('update:modelValue', newVal)
}

const emit = defineEmits(['update:modelValue'])
</script>
