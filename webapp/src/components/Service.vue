<template>
  <div>
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">
      <img :src="logo" class="mr-2 inline h-6 w-6" />
      <span class="capitalize">
        {{ serviceType }}
      </span>
    </h4>
    <div class="flex flex-col space-y-6 rounded-lg bg-white px-4 py-3 shadow-md dark:bg-gray-800">
      <form class="flex flex-col space-y-3" @submit.prevent>
        <InputsServiceGroup :required="true" name="host" label="Host">
          <InputsInput
            v-model="serviceInfo.host"
            type="text"
            name="host"
            :required="true"
            label="Host"
            placeholder="e.g. localhost or 192.168.0.101" />
        </InputsServiceGroup>
        <p v-for="error in v$.host.$errors" :key="error.$uid" class="text-xs text-red-600">
          {{ error.$message }}
        </p>

        <InputsServiceGroup :required="true" name="port" label="Port">
          <InputsInput v-model="serviceInfo.port" type="number" name="port" label="Port" placeholder="e.g. 5050" />
        </InputsServiceGroup>
        <InputsServiceGroup :required="true" name="apiKey" label="API Key">
          <InputsInput
            v-model="serviceInfo.apiKey"
            type="text"
            name="apiKey"
            label="API Key"
            placeholder="e.g. e56a4ls3820483usdu8uf8yur38f8y" />
        </InputsServiceGroup>
        <InputsCheckbox v-model="serviceInfo.useSsl">Use SSL</InputsCheckbox>
        <p v-for="error in v$.apiKey.$errors" :key="error.$uid" class="text-xs text-red-600">
          {{ error.$message }}
        </p>
        <div class="flex justify-end space-x-4">
          <ButtonsStatused class="rounded-lg" :callback="testService">Test</ButtonsStatused>
          <ButtonsStatused class="rounded-lg" :callback="saveService" :is-outlined="false">Save</ButtonsStatused>
        </div>
      </form>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { reactive } from 'vue'
import { PropType } from 'vue'
import { APIResponse, ServiceInfo, Services } from '~/@types/deleterr'
import { useVuelidate } from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
import InputsInput from '~/components/Inputs/Input.vue'
import InputsCheckbox from '~/components/Inputs/Checkbox.vue'
import InputsServiceGroup from '~/components/Inputs/ServiceGroup.vue'
import { useServiceStore } from '~/stores/services.store'
import { ServiceStatus } from '~/@types/services'

const props = defineProps({
  logo: { type: String, required: true },
  service: {
    type: Object as PropType<ServiceInfo>,
    required: false,
  },
  serviceType: {
    type: String as PropType<Services>,
    required: true,
  },
})

const serviceInfo: ServiceInfo = reactive({
  host: props.service?.host ?? '',
  apiKey: props.service?.apiKey ?? '',
  port: props.service?.port ?? '',
  useSsl: props.service?.useSsl ?? false,
  service: props.serviceType,
})

const rules = {
  host: { required },
  apiKey: { required },
}

const store = useServiceStore()

const v$ = useVuelidate(rules, serviceInfo as any)

const saveService = async (): Promise<APIResponse<ServiceStatus> | undefined> => {
  const validation = await v$.value.$validate()
  if (validation) {
    return store.saveService(serviceInfo)
  }
}

const testService = async (): Promise<APIResponse<ServiceStatus> | undefined> => {
  const validation = await v$.value.$validate()
  if (validation) {
    return store.testService(serviceInfo)
  }
}
</script>
