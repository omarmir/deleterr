<template>
  <div>
    <div class="flex flex-row place-items-center justify-between">
      <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">
        <slot name="title"></slot>
      </h2>
      <div v-if="status === 'complete'">
        <RequestsActionsRefresh @click="refreshIndex()"></RequestsActionsRefresh>
      </div>
    </div>
    <PageLoading v-if="status === 'loading'">
      <template #message>Loading</template>
      <template #subtitle>
        <div>Connecting to services and creating index.</div>
        <div class="mt-4" v-if="currProg">{{ currProg.progress }} / {{ currProg.total }}</div>
      </template>
      <template #progress>
        <div class="h-2.5 w-full rounded-full bg-gray-200 dark:bg-gray-700">
          <div class="h-2.5 rounded-full bg-purple-600" :style="`width: ${progress}%`"></div>
        </div>
      </template>
    </PageLoading>
    <Suspense v-else-if="status === 'complete'">
      <RequestsList />
      <template #fallback>
        <PageLoading>
          <template #message>Processing</template>
        </PageLoading>
      </template>
    </Suspense>
    <Error :error="errorStr" v-else></Error>
  </div>
</template>
<script lang="ts" setup>
import { useEventSource } from '@vueuse/core'
import { computed, onUnmounted, ref, Ref, watch } from 'vue'
import { EventsSSE, Progress } from '~/@types/deleterr'
import PageLoading from '~/components/PageLoading.vue'
import RequestsList from '~/components/Requests/List.vue'
import Error from '~/components/Error.vue'
import RequestsActionsRefresh from '~/components/Requests/Actions/Refresh.vue'

const isRefresh = ref(false)

const url = () => (isRefresh.value ? '/api/v1/json/requests/sse/refresh' : '/api/v1/json/requests/sse/get')

const { event, data, close, error } = useEventSource(() => url(), EventsSSE)
const currProg: Ref<Progress | null> = ref(null)
const status: Ref<'error' | 'complete' | 'loading'> = ref('loading')
const errorStr: Ref<string | null> = ref(null)

watch(data, () => {
  if (event.value === 'progress') {
    status.value = 'loading'
    currProg.value = JSON.parse(data.value ?? '') as Progress
  } else if (event.value === 'complete') {
    status.value = 'complete'
    close()
  } else if (event.value === 'error') {
    status.value = 'error'
    errorStr.value = data.value ?? 'Server error'
    close()
  }
})

watch(error, () => {
  status.value = 'error'
  errorStr.value = data.value ?? 'Server error'
  close()
})

const progress = computed(() => {
  if (!currProg.value) return 0
  return (currProg.value.progress / currProg.value.total) * 100
})

const refreshIndex = () => {
  close()
  isRefresh.value = true
  open()
}

onUnmounted(() => close())
</script>
