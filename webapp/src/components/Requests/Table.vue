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
import { useRequests } from '~/composables/useRequests'

const { requests, tableState, resort, getRequests } = useRequests()

await getRequests()
</script>
