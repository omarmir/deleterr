<template>
  <div class="relative block w-full text-gray-500 focus-within:text-purple-600">
    <input
      :id="name"
      v-model="model"
      class="form-input block w-full rounded-md border p-2 pr-20 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
      :placeholder="placeholder"
      :type="type"
      :name="name" />
    <ButtonsStatus
      @click="$emit('click')"
      :is-outlined="false"
      :is-submit
      :provided-operation-state
      class="absolute inset-y-0 right-0 rounded-l-none rounded-r-md border border-transparent bg-purple-600 px-4 text-sm font-medium leading-5 text-white transition-colors duration-150 hover:bg-purple-700 focus:shadow-outline-purple focus:outline-none active:bg-purple-600">
      {{ buttonLabel }}
    </ButtonsStatus>
  </div>
</template>
<script lang="ts" setup>
import { InputType, OperationState } from '~/@types/common'
import ButtonsStatus from '~/components/Buttons/Status.vue'

const {
  providedOperationState,
  isSubmit = false,
  name,
  buttonLabel,
  placeholder,
  type,
} = defineProps<{
  providedOperationState: OperationState
  isSubmit?: boolean
  label: string
  name: string
  buttonLabel: string
  placeholder: string
  required: boolean
  type: InputType
}>()

defineEmits(['click'])

const model = defineModel({ required: false, type: String })
</script>
