<template>
  <button
    class="flex items-center justify-between rounded-lg px-2 py-2 text-sm font-medium leading-5 text-purple-600 focus:shadow-outline-gray focus:outline-none"
    aria-label="Exclude"
    @click="toggleExempt()">
    <IndicatorsLoading v-if="isFetching" />
    <IndicatorsError v-else-if="error || data?.success === false" />
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
</template>
<script lang="ts" setup>
import IndicatorsLoading from '~/components/Indicators/Loading.vue'
import IndicatorsError from '~/components/Indicators/Error.vue'
import { useFetch } from '@vueuse/core'
import { ref, watch } from 'vue'
import { APIResponse, MediaExemptions } from '~/@types/deleterr'
import { useToast } from '~/composables/useToast'

const { requestId, exemptions } = defineProps<{ requestId: number; exemptions: MediaExemptions }>()
const { publishToast } = useToast()

const isExempt = ref(exemptions.includes(requestId) ?? false)

const url = isExempt.value ? '/api/v1/json/request/exemptions/remove' : '/api/v1/json/request/exemptions/save'

console.log(isExempt.value)

const {
  error,
  data,
  execute: toggleExempt,
  isFetching,
  isFinished,
} = useFetch(url, {
  immediate: false,
})
  .post(JSON.stringify(requestId), 'application/json')
  .json<APIResponse<String>>()

watch(isFinished, (newFinished) => {
  if (!newFinished) return

  if (error.value) {
    publishToast('Unable to toggle exemption', 'Error: ' + (error.value as any).toString(), 10, true)
    return
  }

  if (!data.value?.success) {
    publishToast('Unable to toggle exemption', 'Error: ' + data.value?.error_msg, 10, true)
  }

  if (isExempt.value) {
    publishToast(
      'Exemption removed',
      'This media item will be automatically deleted at the next scheduled run.',
      3,
      true
    )
    isExempt.value = false
    emit('exemptionRemoved', requestId)
  } else {
    publishToast('Exempted', 'This media item will not be automatically deleted at next scheduled run.', 3, false)
    isExempt.value = true
    emit('exemptionAdded', requestId)
  }
})

const emit = defineEmits<{
  (e: 'exemptionAdded', id: number): void
  (e: 'exemptionRemoved', id: number): void
}>()
</script>
