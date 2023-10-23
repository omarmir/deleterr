<template>
    <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Dashboard</h2>
    <!-- CTA -->
    <!-- Cards -->
    <!-- New Table -->
    <div class="shadow-xs w-full overflow-hidden rounded-lg">
        <div class="w-full overflow-x-auto">
            <button @click="getRequests()">Get it!</button>
            <Datatable :requests="requests"></Datatable>
        </div>
    </div>
</template>
<script setup lang="ts">
import Datatable from '~/components/Datatable.vue'
import type { Ref } from 'vue'
import { ref } from 'vue'
import { RequestStatus, APIResponse } from '~/@types/deleterr.ts'

const requests: Ref<RequestStatus[]> = ref([])

const getRequests = async () => {
    try {
        const response = await fetch('http://localhost:8080/api/v1/json/requests?take=10&skip=0')
        let api_response: APIResponse<RequestStatus[]> = await response.json()
        requests.value = api_response.data
    } catch (error) {
        console.error(error)
    }
}
</script>
