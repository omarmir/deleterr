<template>
  <div class="shadow-xs w-full overflow-hidden rounded-lg">
    <div class="w-full overflow-x-auto">
      <table class="whitespace-no-wrap w-full">
        <thead>
          <tr
            class="border-b bg-gray-50 text-left text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
            <Heading
              :sort-icon="store.tableState.sortBy === 'name'"
              :is-descending="store.tableState.isDescending"
              @sort-clicked="store.resort('name')">
              Media
            </Heading>
            <Heading
              :sort-icon="store.tableState.sortBy === 'requestedDate'"
              :is-descending="store.tableState.isDescending"
              @sort-clicked="store.resort('requestedDate')">
              Requested
            </Heading>
            <Heading
              :sort-icon="store.tableState.sortBy === 'mediaType'"
              :is-descending="store.tableState.isDescending"
              @sort-clicked="store.resort('mediaType')">
              Type
            </Heading>
            <Heading
              :sort-icon="store.tableState.sortBy === 'watched'"
              :is-descending="store.tableState.isDescending"
              @sort-clicked="store.resort('watched')">
              Watched
            </Heading>
            <Heading
              :sort-icon="store.tableState.sortBy === 'user'"
              :is-descending="store.tableState.isDescending"
              @sort-clicked="store.resort('user')">
              Plex Username
            </Heading>
            <Heading :sort-icon="false">Actions</Heading>
          </tr>
        </thead>
        <tbody class="divide-y bg-white dark:divide-gray-700 dark:bg-gray-800">
          <Row v-for="request in store.requests" :key="request.mediaRequest.id" :request="request" />
        </tbody>
      </table>
      <Pagination
        :take="store.tableState.take"
        :filtered-requests="store.filteredRequests"
        :selected-page="store.currentPage"
        :page-count="store.pageCount ?? 1"
        @change-page="store.changePage"></Pagination>
    </div>
  </div>
</template>
<script lang="ts" setup>
import Row from './Row.vue'
import Heading from './Heading.vue'
import Pagination from '~/components/Pagination.vue'
import { useRequestsStore } from '~/stores/requests'

const store = useRequestsStore()

await store.getRequests()
</script>
