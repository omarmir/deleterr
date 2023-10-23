<template>
    <tr class="text-gray-700 dark:text-gray-400">
        <td class="px-4 py-3">
            <p class="font-semibold">{{ request?.mediaRequest.id }}</p>
        </td>
        <td class="px-4 py-3 text-sm" :data-media-type="request?.mediaRequest.media.mediaType">
            {{ request?.mediaRequest.media.mediaType }}
        </td>
        <td class="px-4 py-3 text-xs">
            <StatusPill :watched-status="request?.userWatchHistory?.watchedStatus" />
        </td>
        <td class="px-4 py-3 text-sm">
            <div class="flex items-center text-sm">
                <!-- Avatar with inset shadow -->
                <div class="relative mr-3 hidden h-8 w-8 rounded-full md:block">
                    <img class="h-full w-full rounded-full object-cover" :src="request?.mediaRequest.requestedBy.avatar"
                        alt="" loading="lazy" />
                    <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true"></div>
                </div>
                <div class="flex flex-col space-y-1">
                    <p class="font-semibold">
                        {{ request?.mediaRequest.requestedBy.plexUsername ??
                            request?.mediaRequest.requestedBy.username }}
                    </p>
                    <p class="text-xs">
                        <span :data-user-type="request?.mediaRequest.requestedBy.userType"
                            class="text-white px-2 py-[2px] rounded-md">
                            {{ userType(request?.mediaRequest.requestedBy.userType) }}
                        </span>
                    </p>
                </div>
            </div>
        </td>
        <Actions />
    </tr>
</template>
<script lang="ts" setup>
import { PropType } from 'vue'
import { RequestStatus } from '~/@types/deleterr.ts'
import StatusPill from '~/components/StatusPill.vue'
import Actions from '~/components/Actions.vue'

defineProps({
    request: { required: false, type: Object as PropType<RequestStatus> },
})

const userType = (userType?: number): 'Local User' | 'Plex User' => userType == 1 ? 'Plex User' : 'Local User'
</script>
<style lang="postcss" scoped>
[data-user-type='1'] {
    @apply bg-purple-600;
}

[data-user-type='2'] {
    @apply bg-teal-600;
}
</style>