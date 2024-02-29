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
        <InputsInput v-model="serviceInfo.host" :required="true" label="Host" placeholder="Host"></InputsInput>
        <p v-for="error in v$.host.$errors" :key="error.$uid" class="text-xs text-red-600">
          {{ error.$message }}
        </p>
        <InputsInput v-model="serviceInfo.port" label="Port" placeholder="Port" type="number"></InputsInput>
        <InputsInput v-model="serviceInfo.apiKey" :required="true" label="API Key" placeholder="API Key"></InputsInput>
        <p v-for="error in v$.apiKey.$errors" :key="error.$uid" class="text-xs text-red-600">
          {{ error.$message }}
        </p>
        <InputsCheckbox v-model="serviceInfo.useSsl">Use SSL</InputsCheckbox>
        <div class="flex justify-end space-x-4">
          <ButtonsStatused class="rounded-lg" :operation-state="operationState" @click="testService">
            Test
          </ButtonsStatused>
          <ButtonsStatused
            class="rounded-lg"
            :operation-state="operationState"
            :is-outlined="false"
            @click="saveService">
            Save
          </ButtonsStatused>
        </div>
        <div class="text-sm text-red-600 first-letter:uppercase">
          <p v-if="operationState == OperationState.failure">
            {{ serviceError }}
          </p>
        </div>
      </form>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { Ref, reactive, ref } from 'vue'
import { PropType } from 'vue'
import { ServiceInfo, Services } from '~/@types/deleterr'
import { useVuelidate } from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
import InputsInput from '~/components/Inputs/Input.vue'
import InputsCheckbox from '~/components/Inputs/Checkbox.vue'
import { useServiceStore } from '~/stores/services.store'
import { OperationState } from '~/@types/common'

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

const operationState: Ref<OperationState> = ref(OperationState.hidden)
const serviceError: Ref<undefined | string> = ref(undefined)

const v$ = useVuelidate(rules, serviceInfo as any)

const saveService = async () => {
  const validation = await v$.value.$validate()
  if (validation) {
    operationState.value = OperationState.loading
    const result = await store.saveService(serviceInfo)

    if (result.success) {
      operationState.value = OperationState.success
    } else {
      operationState.value = OperationState.failure
    }

    serviceError.value = result.error_msg

    setTimeout(() => {
      operationState.value = OperationState.hidden
    }, 5000)
  }
}

const testService = async () => {
  const validation = await v$.value.$validate()
  if (validation) {
    operationState.value = OperationState.loading
    const result = await store.saveService(serviceInfo)

    if (result.success) {
      operationState.value = OperationState.success
    } else {
      operationState.value = OperationState.failure
    }

    serviceError.value = result.error_msg

    setTimeout(() => {
      operationState.value = OperationState.hidden
    }, 5000)
  }
}
</script>
