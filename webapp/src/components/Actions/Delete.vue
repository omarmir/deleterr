<template>
  <button
    v-if="externalId"
    class="flex items-center justify-between rounded-lg px-2 py-2 text-sm font-medium leading-5 text-red-500 focus:shadow-outline-gray focus:outline-none"
    aria-label="Delete"
    @click="deleteMedia()">
    <IndicatorsLoading v-if="isFetching" />
    <IndicatorsError v-else-if="error || data?.success === false" />
    <svg v-else class="h-5 w-5" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20">
      <path
        fill-rule="evenodd"
        d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
        clip-rule="evenodd"></path>
    </svg>
  </button>
</template>
<script lang="ts" setup>
import IndicatorsLoading from '~/components/Indicators/Loading.vue'
import IndicatorsError from '~/components/Indicators/Error.vue'
import { APIResponse, MovieDeletionRequest } from '~/@types/deleterr'
import { useFetch } from '@vueuse/core'
import { watch } from 'vue'
import { useToast } from '~/composables/useToast'

const { requestId, externalId } = defineProps<{ requestId: number; externalId: number }>()
const { publishToast } = useToast()

const {
  error,
  data,
  execute: deleteMedia,
  isFetching,
  isFinished,
} = useFetch(`/api/v1/json/movie/delete/${requestId}`, {
  refetch: true,
  immediate: false,
})
  .delete()
  .json<APIResponse<MovieDeletionRequest>>()

watch(isFinished, (newFinished) => {
  if (!newFinished) return

  if (error.value) {
    publishToast('Unable to delete movie', 'Error: ' + (error.value as any).toString(), 10, true)
    return
  }

  if (data.value?.success) {
    publishToast('Movie deleted', 'Movie has been deleted! You may need to re-scan on plex for it to vanish', 5, false)
    emit('delete', requestId)
  } else {
    publishToast('Unable to delete movie', 'Error: ' + data.value?.error_msg, 10, true)
  }
})

const emit = defineEmits<{
  (e: 'delete', id: number): void
}>()
</script>
