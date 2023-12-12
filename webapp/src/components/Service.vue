<template>
  <div>
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">
      <img :src="logo" class="mr-2 inline h-6 w-6" />
      <span class="capitalize">
        {{ serviceType }}
      </span>
    </h4>
    <div class="flex flex-col space-y-6 rounded-lg bg-white px-4 py-3 shadow-md dark:bg-gray-800">
      <form class="flex flex-col space-y-3" @submit.prevent="saveOrTestService(ServiceOperations.Save)">
        <InputsInput v-model="serviceInfo.host" :required="true" label="Host" placeholder="Host"></InputsInput>
        <p v-for="error in v$.host.$errors" :key="error.$uid" class="text-xs text-red-600">
          {{ error.$message }}
        </p>
        <InputsInput v-model="serviceInfo.port" label="Port" placeholder="Port" :type="InputType.number"></InputsInput>
        <InputsInput v-model="serviceInfo.apiKey" :required="true" label="API Key" placeholder="API Key"></InputsInput>
        <p v-for="error in v$.apiKey.$errors" :key="error.$uid" class="text-xs text-red-600">
          {{ error.$message }}
        </p>
        <InputsCheckbox v-model="serviceInfo.useSsl">Use SSL</InputsCheckbox>
        <div class="flex justify-end space-x-4">
          <ButtonsStatused
            :button-state="serviceStatus?.test"
            class="rounded-lg"
            @click="saveOrTestService(ServiceOperations.Test)">
            Test
          </ButtonsStatused>
          <ButtonsStatused
            :button-state="serviceStatus?.save"
            :is-submit="true"
            :is-outlined="false"
            class="rounded-lg">
            Save
          </ButtonsStatused>
        </div>
        <div class="text-sm text-red-600 first-letter:uppercase">
          <p v-if="serviceStatus?.save === OperationState.failure">
            {{ serviceStatus?.errorMsg }}
          </p>
          <p v-if="serviceStatus?.test === OperationState.failure">
            {{ serviceStatus?.errorMsg }}
          </p>
        </div>
      </form>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { reactive } from 'vue'
import { PropType } from 'vue'
import { ServiceInfo, Services } from '~/@types/deleterr'
import { InputType, OperationState } from '~/@types/common'
import { ServiceOperations, ServiceStatus } from '~/@types/services'
import { useVuelidate } from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
import InputsInput from '~/components/Inputs/Input.vue'
import InputsCheckbox from '~/components/Inputs/Checkbox.vue'

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
  serviceStatus: {
    type: Object as PropType<ServiceStatus>,
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

const emit = defineEmits<{
  (e: 'save', serviceInfo: ServiceInfo): void
  (e: 'test', serviceInfo: ServiceInfo): void
}>()

const saveOrTestService = async (operation: ServiceOperations) => {
  const result = await v$.value.$validate()

  if (result) operation === ServiceOperations.Test ? emit('test', serviceInfo) : emit('save', serviceInfo)
}

const v$ = useVuelidate(rules, serviceInfo as any)
</script>
