<template>
  <div>
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">
      <img :src="logo" class="mr-2 inline h-6 w-6" />
      <span class="capitalize">{{ serviceType }}</span>
    </h4>
    <div class="flex flex-col space-y-6 rounded-lg bg-white px-4 py-3 shadow-md dark:bg-gray-800">
      <form class="flex flex-col space-y-3" @submit.prevent>
        <InputsServiceGroup :required="true" name="host" label="Host" :errors="v$.host.$errors">
          <InputsInput
            v-model="service.host"
            type="text"
            name="host"
            :required="true"
            label="Host"
            placeholder="e.g. localhost or 192.168.0.101" />
        </InputsServiceGroup>
        <InputsServiceGroup :required="false" name="port" label="Port" :errors="[]">
          <InputsInput v-model="service.port" type="number" name="port" label="Port" placeholder="e.g. 5050" />
        </InputsServiceGroup>
        <InputsServiceGroup :required="true" name="apiKey" label="API Key" :errors="v$.apiKey.$errors">
          <InputsInput
            v-model="service.apiKey"
            type="text"
            name="apiKey"
            label="API Key"
            placeholder="e.g. e56a4ls3820483usdu8uf8yur38f8y" />
        </InputsServiceGroup>
        <InputsCheckbox v-model="service.useSsl">Use SSL</InputsCheckbox>
        <div class="flex justify-end space-x-4">
          <ButtonsStatus @click="testService" :provided-operation-state="testingState">Test</ButtonsStatus>
          <ButtonsStatus @click="saveService" :is-outlined="false" :provided-operation-state="savingState">
            Save
          </ButtonsStatus>
        </div>
      </form>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { ServiceInfo, Services } from '~/@types/deleterr'
import { useVuelidate } from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import ButtonsStatus from '~/components/Buttons/Status.vue'
import InputsInput from '~/components/Inputs/Input.vue'
import InputsCheckbox from '~/components/Inputs/Checkbox.vue'
import InputsServiceGroup from '~/components/Inputs/ServiceGroup.vue'
import { useService } from '~/composables/useService'

const { logo, serviceInfo, serviceType } = defineProps<{
  logo: string
  serviceInfo?: ServiceInfo
  serviceType: Services
}>()

const rules = {
  host: { required },
  apiKey: { required },
}

const { test, save, testingState, savingState, service } = useService(serviceType, serviceInfo)

const v$ = useVuelidate(rules, service as any)

const saveService = async () => {
  const validation = await v$.value.$validate()
  if (validation) save()
}

const testService = async () => {
  const validation = await v$.value.$validate()
  if (validation) test()
}
</script>
