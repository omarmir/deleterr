<template>
    <table class="whitespace-no-wrap w-full">
        <thead>
            <tr
                class="border-b bg-gray-50 text-left text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
                <th class="px-4 py-3">ID</th>
                <th class="px-4 py-3">Type</th>
                <th class="px-4 py-3">Watched</th>
                <th class="px-4 py-3">Plex Username</th>
            </tr>
        </thead>
        <tbody class="divide-y bg-white dark:divide-gray-700 dark:bg-gray-800">
            <DatatableRow />
        </tbody>
    </table>
</template>
<script lang="ts" setup>
import DatatableRow from '~/components/DatatableRow.vue'
</script>
