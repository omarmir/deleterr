<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Dashboard</h2>
  <PageLoading v-if="loadingState === 'loading'">
    <template #message>Loading</template>
    <template #subtitle>
      <div>Connecting to services and creating index.</div>
      <div v-if="currProg">{{ currProg.progress }} / {{ currProg.total }}</div>
    </template>
    <template #progress>
      <div class="h-2.5 w-full rounded-full bg-gray-200 dark:bg-gray-700">
        <div class="h-2.5 rounded-full bg-purple-600" :style="`width: ${progress}%`"></div>
      </div>
    </template>
  </PageLoading>
  <Suspense v-else-if="loadingState === 'complete'">
    <RequestsList />
    <template #fallback>
      <PageLoading>
        <template #message>Processing</template>
      </PageLoading>
    </template>
  </Suspense>
  <Error :error="errorStr" v-else></Error>
</template>
<script setup lang="ts">
import RequestsList from '~/components/Requests/List.vue'
import PageLoading from '~/components/PageLoading.vue'
import Error from '~/components/Error.vue'
import { useEventSource } from '@vueuse/core'
import { EventsSSE, Progress } from '~/@types/deleterr'
import { Ref, ref, watch } from 'vue'

const loadingState: Ref<'loading' | 'complete' | 'error'> = ref('loading')
const progress = ref(0)
const currProg: Ref<Progress | null> = ref(null)
const errorStr: Ref<string | null> = ref(null)

const { event, data, close } = useEventSource('/api/v1/json/requests/sse', EventsSSE)

watch(data, () => {
  console.log(data)
  if (event.value === 'progress') {
    currProg.value = JSON.parse(data.value ?? '') as Progress
    progress.value = (currProg.value.progress / currProg.value.total) * 100
  } else if (event.value === 'completion') {
    loadingState.value = 'complete'
    close()
  } else if (event.value === 'error') {
    errorStr.value = data.value ?? 'Server error'
  }
})
</script>
