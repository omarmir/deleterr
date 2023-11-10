<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Services</h2>
  <Suspense>
    <AllServices />
    <template #fallback>
      <PageLoading>
        <template #message>Loading.</template>
      </PageLoading>
    </template>
  </Suspense>
</template>
<script lang="ts" setup async>
import AllServices from '~/components/AllServices.vue'
import PageLoading from '~/components/PageLoading.vue'
</script>
