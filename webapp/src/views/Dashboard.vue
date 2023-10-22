import Table from '../components/Table.vue';
<template>
    <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Dashboard</h2>
    <!-- CTA -->
    <!-- Cards -->
    <!-- New Table -->
    <div class="shadow-xs w-full overflow-hidden rounded-lg">
        <div class="w-full overflow-x-auto">
            <Datatable></Datatable>
        </div>
    </div>
</template>
<script setup lang="ts">
import Datatable from '~/components/Datatable.vue'
</script>
