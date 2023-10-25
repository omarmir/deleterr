<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Dashboard</h2>
  <div class="mb-4">
    <ButtonsRegular @click="getRequests()">Get it!</ButtonsRegular>
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
import ButtonsRegular from '~/components/Buttons/Regular.vue'

const requests: Ref<RequestStatus[]> = ref([])
const allRequests: Ref<number> = ref(0)

const getRequests = async () => {
  try {
    const response = await fetch('http://localhost:8080/api/v1/json/requests?take=11&skip=0')
    let apiResponse: APIResponse<RequestStatusWithRecordInfo> = await response.json()
    requests.value = apiResponse.data.requests
    allRequests.value = apiResponse.data.allRequests
  } catch (error) {
    console.error(error)
  }
}
</script>
