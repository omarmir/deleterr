<template>
  <div class="shadow-xs w-full overflow-hidden rounded-lg">
    <div class="w-full overflow-x-auto">
      <table class="whitespace-no-wrap w-full">
        <thead>
          <tr class="border-b bg-gray-50 text-left text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
            <Heading>Media</Heading>
            <Heading>Requested</Heading>
            <Heading>Type</Heading>
            <Heading>Watched</Heading>
            <Heading>Plex Username</Heading>
            <Heading :sort-icon="false">Actions</Heading>
          </tr>
        </thead>
        <tbody class="divide-y bg-white dark:divide-gray-700 dark:bg-gray-800">
          <Row v-for="request in requests" :key="request.mediaRequest.id" :request="request" />
        </tbody>
      </table>
    </div>
  </div>
</template>
<script lang="ts" setup>
import Row from './Row.vue'
import Heading from './Heading.vue'
import type { Ref } from 'vue'
import { ref } from 'vue'
import { RequestStatusWithRecordInfo, RequestStatus, APIResponse } from '~/@types/deleterr.ts'

const requests: Ref<RequestStatus[] | undefined> = ref([])
const allRequests: Ref<number> = ref(0)

const getRequests = async () => {
  try {
    const response = await fetch('http://localhost:8080/api/v1/json/requests')
    let apiResponse: APIResponse<RequestStatusWithRecordInfo> = await response.json()
    requests.value = apiResponse.data?.requests
    allRequests.value = apiResponse.data?.allRequests ?? 0
  } catch (error) {
    console.error(error)
  }
}

await getRequests()
</script>
