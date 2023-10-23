<template>
    <tr class="text-gray-700 dark:text-gray-400">
        <td class="px-4 py-3">
            <div class="flex items-center text-sm">
                <!-- Avatar with inset shadow -->
                <div class="relative mr-3 hidden h-8 w-8 rounded-full md:block">
                    <img
                        class="h-full w-full rounded-full object-cover"
                        src="https://images.unsplash.com/flagged/photo-1570612861542-284f4c12e75f?ixlib=rb-1.2.1&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=200&fit=max&ixid=eyJhcHBfaWQiOjE3Nzg0fQ"
                        alt=""
                        loading="lazy" />
                    <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true"></div>
                </div>
                <div>
                    <p class="font-semibold">{{ request?.mediaRequest.id }}</p>
                    <p class="text-xs text-gray-600 dark:text-gray-400">10x Developer</p>
                </div>
            </div>
        </td>
        <td class="px-4 py-3 text-sm">$ 863.45</td>
        <td class="px-4 py-3 text-xs">
            <StatusPill :watched-status="1" />
        </td>
        <td class="px-4 py-3 text-sm">6/10/2020</td>
    </tr>
</template>
<script lang="ts" setup>
import StatusPill from '~/components/StatusPill.vue'
import { PropType } from 'vue'
import { RequestStatus } from '~/@types/deleterr.ts'

defineProps({
    request: { required: false, type: Object as PropType<RequestStatus> },
})
</script>
