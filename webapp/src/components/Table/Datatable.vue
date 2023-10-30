<template>
  <div class="shadow-xs w-full overflow-hidden rounded-lg">
    <div class="w-full overflow-x-auto">
      <table class="whitespace-no-wrap w-full">
        <thead>
          <tr
            class="border-b bg-gray-50 text-left text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
            <th class="px-4 py-3">Media</th>
            <th class="px-4 py-3">Requested</th>
            <th class="px-4 py-3">Type</th>
            <th class="px-4 py-3">Watched</th>
            <th class="px-4 py-3">Plex Username</th>
            <th class="px-4 py-3">Actions</th>
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
import DatatableRow from '~/components/Table/DatatableRow.vue'
import { RequestStatus } from '~/@types/deleterr.ts'

defineProps({
  requests: { required: false, type: Array as PropType<RequestStatus[]> },
})
</script>
