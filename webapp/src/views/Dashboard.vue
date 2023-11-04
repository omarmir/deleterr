<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Dashboard</h2>
  <div class="mb-4">
    <ButtonsBase @click="getRequests()">Get it!</ButtonsBase>
    <div class="mt-2 text-sm text-gray-700 dark:text-gray-200">total records: {{ allRequests }}</div>
    <div class="mt-2 text-sm text-gray-700 dark:text-gray-200">total pages: {{ Math.ceil(allRequests / 10) }}</div>
  </div>
  <Datatable :requests="requests"></Datatable>
</template>
<script setup lang="ts">
import Datatable from '~/components/Table/Datatable.vue'
import type { Ref } from 'vue'
import { ref } from 'vue'
import { RequestStatusWithRecordInfo, RequestStatus, APIResponse } from '~/@types/deleterr.ts'
import ButtonsBase from '~/components/Buttons/Base.vue'

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
</script>
