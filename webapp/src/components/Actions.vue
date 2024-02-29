<template>
  <div class="flex items-center space-x-4 text-sm">
    <button
      class="flex items-center justify-between rounded-lg px-2 py-2 text-sm font-medium leading-5 text-purple-600 focus:shadow-outline-gray focus:outline-none"
      aria-label="Exclude"
      @click="exemptMedia">
      <IndicatorsLoading v-if="exemptionState === OperationState.loading" />
      <IndicatorsError v-else-if="exemptionState === OperationState.failure" />
      <svg
        v-else
        class="h-5 w-5"
        :fill="isExempt ? 'currentColor' : 'none'"
        stroke="currentColor"
        stroke-width="1.5"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0111.186 0z"></path>
      </svg>
    </button>
    <button
      v-if="externalId"
      class="flex items-center justify-between rounded-lg px-2 py-2 text-sm font-medium leading-5 text-red-500 focus:shadow-outline-gray focus:outline-none"
      aria-label="Delete"
      @click="deleteMedia">
      <IndicatorsLoading v-if="deletionState === OperationState.loading" />
      <IndicatorsError v-else-if="deletionState === OperationState.failure" />
      <svg v-else class="h-5 w-5" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20">
        <path
          fill-rule="evenodd"
          d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
          clip-rule="evenodd"></path>
      </svg>
    </button>
  </div>
</template>
<script lang="ts" setup>
import { PropType, Ref, ref } from 'vue'
import { OperationState } from '~/@types/common'
import IndicatorsLoading from '~/components/Indicators/Loading.vue'
import IndicatorsError from '~/components/Indicators/Error.vue'
import { APIResponse } from '~/@types/deleterr'

const props = defineProps({
  isExempt: { required: true, type: Boolean, default: false },
  externalId: { type: Number, required: false },
  deletionCallback: {
    type: Function as PropType<() => Promise<APIResponse<any> | undefined>>,
    required: true,
  },
  exemptionCallback: {
    type: Function as PropType<() => Promise<APIResponse<any> | undefined>>,
    required: true,
  },
})

const deletionState: Ref<OperationState> = ref(OperationState.hidden)
const exemptionState: Ref<OperationState> = ref(OperationState.hidden)

const deleteMedia = async () => {
  deletionState.value = OperationState.loading

  const result = await props.deletionCallback()

  if (result) {
    result.success ? (deletionState.value = OperationState.success) : (deletionState.value = OperationState.failure)
    setTimeout(() => {
      deletionState.value = OperationState.hidden
    }, 5000)
  } else {
    deletionState.value = OperationState.hidden
  }
}

const exemptMedia = async () => {
  exemptionState.value = OperationState.loading

  const result = await props.exemptionCallback()

  if (result) {
    result.success ? (exemptionState.value = OperationState.success) : (exemptionState.value = OperationState.failure)
    setTimeout(() => {
      exemptionState.value = OperationState.hidden
    }, 5000)
  } else {
    exemptionState.value = OperationState.hidden
  }
}
</script>
