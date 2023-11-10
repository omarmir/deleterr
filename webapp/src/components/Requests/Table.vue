<template>
  <div class="shadow-xs w-full overflow-hidden rounded-lg">
    <div v-if="store.error" class="text-red-400">
      {{ store.error.toString() }}
    </div>
    <div v-if="!store.error" class="w-full overflow-x-auto">
      <table class="whitespace-no-wrap w-full table-fixed">
        <thead>
          <tr
            class="border-b bg-gray-50 text-left text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
            <Heading class="w-1/3" name="name" :table-state="store.tableState" @sort-clicked="store.resort">
              Media
            </Heading>
            <Heading name="requestedDate" :table-state="store.tableState" @sort-clicked="store.resort">
              Requested
            </Heading>
            <Heading name="mediaType" :table-state="store.tableState" @sort-clicked="store.resort">Type</Heading>
            <Heading name="watched" :table-state="store.tableState" @sort-clicked="store.resort">Watched</Heading>
            <Heading name="user" :table-state="store.tableState" @sort-clicked="store.resort">Username</Heading>
            <Heading name="actions" :table-state="store.tableState">Actions</Heading>
          </tr>
        </thead>
        <tbody class="divide-y bg-white dark:divide-gray-700 dark:bg-gray-800">
          <Row
            v-for="request in store.requests"
            :key="request.mediaRequest.id"
            :request="request"
            :is-exempt="store.mediaExemptions.hasOwnProperty(request.mediaRequest.id)" />
        </tbody>
      </table>

      {{ store.mediaExemptions }}
      <PaginationWrapper
        :take="store.tableState.take"
        :filtered-requests="store.filteredRequests"
        :selected-page="store.currentPage"
        :page-count="store.pageCount ?? 1"
        @change-page="store.changePage" />
    </div>
  </div>
</template>
<script lang="ts" setup>
import Row from './Row.vue'
import Heading from './Heading.vue'
import PaginationWrapper from '~/components/Pagination/Wrapper.vue'
import { useRequestsStore } from '~/stores/requests'

const store = useRequestsStore()

await store.getRequests()
</script>
