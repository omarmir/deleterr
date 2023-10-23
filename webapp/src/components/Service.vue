<template>
  <div class="basis-1/2">
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">
      <img :src="logo" class="mr-2 inline h-6 w-6" />
      <slot></slot>
    </h4>
    <div
      class="mb-8 flex flex-col space-y-6 rounded-lg bg-white px-4 py-3 shadow-md dark:bg-gray-800">
      <InputsInput label="Host" placeholder="Host"></InputsInput>
      <InputsInput
        label="Port"
        placeholder="Port"
        :type="InputType.number"></InputsInput>
      <InputsInput label="API Key" placeholder="API Key"></InputsInput>
      <InputsCheckbox>Use SSL</InputsCheckbox>
      <div class="flex justify-end space-x-4">
        <ButtonsOutline>Test</ButtonsOutline>
        <ButtonsRegular>Save</ButtonsRegular>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
defineProps({
  logo: { type: String, required: true },
})
import InputsInput from '~/components/Inputs/Input.vue'
import InputsCheckbox from '~/components/Inputs/Checkbox.vue'
import { InputType } from '~/@types/deleterr.ts'
import ButtonsRegular from '~/components/Buttons/Regular.vue'
import ButtonsOutline from '~/components/Buttons/Outline.vue'
</script>
