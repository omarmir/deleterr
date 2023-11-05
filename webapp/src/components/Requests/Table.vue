<template>
  <div class="shadow-xs w-full overflow-hidden rounded-lg">
    <div class="w-full overflow-x-auto">
      <table class="whitespace-no-wrap w-full">
        <thead>
          <tr class="border-b bg-gray-50 text-left text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
            <Heading :sort-icon="tableState.sortBy === 'name'" :is-descending="tableState.isDescending" @sort-clicked="resort('name')">Media</Heading>
            <Heading :sort-icon="tableState.sortBy === 'requestedDate'" :is-descending="tableState.isDescending" @sort-clicked="resort('requestedDate')">Requested</Heading>
            <Heading :sort-icon="tableState.sortBy === 'mediaType'" :is-descending="tableState.isDescending" @sort-clicked="resort('mediaType')">Type</Heading>
            <Heading :sort-icon="tableState.sortBy === 'watched'" :is-descending="tableState.isDescending" @sort-clicked="resort('watched')">Watched</Heading>
            <Heading :sort-icon="tableState.sortBy === 'user'" :is-descending="tableState.isDescending" @sort-clicked="resort('user')">Plex Username</Heading>
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
import { reactive, ref } from 'vue'
import { RequestStatusWithRecordInfo, RequestStatus, APIResponse } from '~/@types/deleterr.ts'

const requests: Ref<RequestStatus[] | undefined> = ref([])
const allRequests: Ref<number> = ref(0)

type Sortables = 'name' | 'requestedDate' | 'mediaType' | 'watched' | 'user'

interface TableState {
  sortBy?: Sortables
  isDescending?: boolean
  take?: number
  skip?: number
}

const tableState: TableState = reactive({
  sortBy: 'requestedDate',
  isDescending: true,
  take: 100,
  skip: undefined,
})

const resort = (sorter: Sortables) => {
  if (tableState.sortBy === sorter) {
    tableState.isDescending = !tableState.isDescending
  } else {
    ;[tableState.sortBy, tableState.isDescending] = [sorter, false]
  }
  getRequests()
}

const getRequests = async () => {
  try {
    const queryString = Object.entries(tableState)
      .filter(([_key, value]) => value !== undefined) // Filter out properties with undefined values
      .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
      .join('&')

    const urlWithQueryParams = `http://localhost:8080/api/v1/json/requests?${queryString}`

    const response = await fetch(urlWithQueryParams)
    let apiResponse: APIResponse<RequestStatusWithRecordInfo> = await response.json()
    requests.value = apiResponse.data?.requests
    allRequests.value = apiResponse.data?.allRequests ?? 0
  } catch (error) {
    console.error(error)
  }
}

await getRequests()
</script>
