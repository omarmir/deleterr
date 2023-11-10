<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Dashboard</h2>
  <Suspense>
    <RequestsTable />
    <template #fallback>
      <PageLoading>
        <template #message>
          Loading. If this is the first run and you have many requests, this may take a while.
        </template>
        <template #subtitle>
          This could timeout and you may want to refresh after a sufficiently long time. In future releases an indicator
          will show progress.
        </template>
      </PageLoading>
    </template>
  </Suspense>
</template>
<script setup lang="ts">
import RequestsTable from '~/components/Requests/Table.vue'
import PageLoading from '~/components/PageLoading.vue'
</script>
