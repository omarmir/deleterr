<template>
  <div class="shadow-xs w-full overflow-hidden rounded-lg">
    <div class="w-full overflow-x-auto">
      <table class="whitespace-no-wrap w-full">
        <thead>
          <tr
            class="border-b bg-gray-50 text-left text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
            <TableHeading>Media</TableHeading>
            <TableHeading>Requested</TableHeading>
            <TableHeading>Type</TableHeading>
            <TableHeading>Watched</TableHeading>
            <TableHeading>Plex Username</TableHeading>
            <TableHeading :sort-icon="false">Actions</TableHeading>
          </tr>
        </thead>
        <tbody class="divide-y bg-white dark:divide-gray-700 dark:bg-gray-800">
          <DatatableRow v-for="request in requests" :key="request.mediaRequest.id" :request="request" />
        </tbody>
      </table>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'
import { RequestStatus } from '~/@types/deleterr.ts'
import DatatableRow from '~/components/Table/DatatableRow.vue'
import TableHeading from '~/components/Table/Heading.vue'

defineProps({
  requests: { required: false, type: Array as PropType<RequestStatus[]> },
})
</script>
