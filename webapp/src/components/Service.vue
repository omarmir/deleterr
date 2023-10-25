<template>
  <div>
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">
      <img :src="logo" class="mr-2 inline h-6 w-6" />
      <slot></slot>
    </h4>
    <div class="flex flex-col space-y-6 rounded-lg bg-white px-4 py-3 shadow-md dark:bg-gray-800">
      <InputsInput v-model="serviceInfo.host" label="Host" placeholder="Host"></InputsInput>
      <InputsInput v-model="serviceInfo.port" label="Port" placeholder="Port" :type="InputType.number"></InputsInput>
      <InputsInput v-model="serviceInfo.apiKey" label="API Key" placeholder="API Key"></InputsInput>
      <InputsCheckbox v-model="serviceInfo.useSsl">Use SSL</InputsCheckbox>
      <div class="flex justify-end space-x-4">
        <ButtonsOutline @click="testService(serviceInfo)">Test</ButtonsOutline>
        <ButtonsRegular>Save</ButtonsRegular>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import type { Ref } from 'vue'
import { ref } from 'vue'
import { ServiceInfo } from '~/@types/deleterr'
import { useServiceTest } from '~/composables/useServiceTest'
import InputsInput from '~/components/Inputs/Input.vue'
import InputsCheckbox from '~/components/Inputs/Checkbox.vue'
import { InputType } from '~/@types/deleterr.ts'
import ButtonsRegular from '~/components/Buttons/Regular.vue'
import ButtonsOutline from '~/components/Buttons/Outline.vue'

const serviceInfo: Ref<ServiceInfo> = ref({
  host: '192.168.2.102',
  apiKey: '',
  port: '5055',
  useSsl: false,
  service: 'overseerr',
})

defineProps({
  logo: { type: String, required: true },
})

const { testService } = useServiceTest()
</script>
