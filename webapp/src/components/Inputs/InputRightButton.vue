<template>
  <div class="relative block w-full text-gray-500 focus-within:text-purple-600">
    <input
      :id="name"
      v-model="model"
      class="form-input block w-full rounded-md border p-2 pr-20 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
      :placeholder="placeholder"
      :type="type"
      :name="name" />
    <ButtonsStatused
      :callback="callback"
      :is-outlined="false"
      class="absolute inset-y-0 right-0 rounded-r-md border border-transparent bg-purple-600 px-4 text-sm font-medium leading-5 text-white transition-colors duration-150 hover:bg-purple-700 focus:shadow-outline-purple focus:outline-none active:bg-purple-600"
      :is-submit="true">
      {{ buttonLabel }}
    </ButtonsStatused>
  </div>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'
import { InputType } from '~/@types/common'
import { APIResponse } from '~/@types/deleterr'
import ButtonsStatused from '~/components/Buttons/Statused.vue'

defineProps({
  label: {
    type: String,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
  buttonLabel: {
    type: String,
    required: true,
  },
  placeholder: String,
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
  callback: {
    type: Function as PropType<() => Promise<APIResponse<any> | undefined>>,
    required: true,
  },
})

const model = defineModel({ type: String })
</script>
