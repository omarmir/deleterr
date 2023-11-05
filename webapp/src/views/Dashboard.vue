<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Dashboard</h2>
  <Suspense>
    <RequestsTable></RequestsTable>
  </Suspense>
</template>
<script setup lang="ts">
import RequestsTable from '~/components/Requests/Table.vue'
</script>
