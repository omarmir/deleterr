<template>
  <tr class="h-24 text-gray-700 dark:text-gray-400">
    <td class="px-4 py-3">
      <div class="flex items-center space-x-5 text-sm">
        <img
          class="h-16 object-cover"
          :src="'https://image.tmdb.org/t/p/w600_and_h900_bestv2' + request?.mediaInfo?.posterPath"
          alt=""
          loading="lazy" />
        <div class="flex flex-col space-y-1">
          <p class="font-semibold">{{ request?.mediaInfo?.title ?? 'N/A' }}</p>
          <p class="text-xs">Release: {{ request?.mediaInfo?.releaseDate }}</p>
        </div>
      </div>
    </td>
    <td class="px-4 py-3">
      <p class="text-sm">{{ getDate(request?.mediaRequest.createdAt) }}</p>
      <p class="text-xs">{{ getTime(request?.mediaRequest.createdAt) }}</p>
    </td>
    <td class="px-4 py-3 text-sm" :data-media-type="request?.mediaRequest.media.mediaType">
      <div class="h-5 w-5 text-center">
        <svg
          v-if="request?.mediaRequest.media.mediaType === 'tv'"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
          aria-hidden="true">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M6 20.25h12m-7.5-3v3m3-3v3m-10.125-3h17.25c.621 0 1.125-.504 1.125-1.125V4.875c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125z"></path>
        </svg>
        <svg
          v-else
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
          aria-hidden="true">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 01-1.125-1.125M3.375 19.5h1.5C5.496 19.5 6 18.996 6 18.375m-3.75 0V5.625m0 12.75v-1.5c0-.621.504-1.125 1.125-1.125m18.375 2.625V5.625m0 12.75c0 .621-.504 1.125-1.125 1.125m1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125m0 3.75h-1.5A1.125 1.125 0 0118 18.375M20.625 4.5H3.375m17.25 0c.621 0 1.125.504 1.125 1.125M20.625 4.5h-1.5C18.504 4.5 18 5.004 18 5.625m3.75 0v1.5c0 .621-.504 1.125-1.125 1.125M3.375 4.5c-.621 0-1.125.504-1.125 1.125M3.375 4.5h1.5C5.496 4.5 6 5.004 6 5.625m-3.75 0v1.5c0 .621.504 1.125 1.125 1.125m0 0h1.5m-1.5 0c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125m1.5-3.75C5.496 8.25 6 7.746 6 7.125v-1.5M4.875 8.25C5.496 8.25 6 8.754 6 9.375v1.5m0-5.25v5.25m0-5.25C6 5.004 6.504 4.5 7.125 4.5h9.75c.621 0 1.125.504 1.125 1.125m1.125 2.625h1.5m-1.5 0A1.125 1.125 0 0118 7.125v-1.5m1.125 2.625c-.621 0-1.125.504-1.125 1.125v1.5m2.625-2.625c.621 0 1.125.504 1.125 1.125v1.5c0 .621-.504 1.125-1.125 1.125M18 5.625v5.25M7.125 12h9.75m-9.75 0A1.125 1.125 0 016 10.875M7.125 12C6.504 12 6 12.504 6 13.125m0-2.25C6 11.496 5.496 12 4.875 12M18 10.875c0 .621-.504 1.125-1.125 1.125M18 10.875c0 .621.504 1.125 1.125 1.125m-2.25 0c.621 0 1.125.504 1.125 1.125m-12 5.25v-5.25m0 5.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125m-12 0v-1.5c0-.621-.504-1.125-1.125-1.125M18 18.375v-5.25m0 5.25v-1.5c0-.621.504-1.125 1.125-1.125M18 13.125v1.5c0 .621.504 1.125 1.125 1.125M18 13.125c0-.621.504-1.125 1.125-1.125M6 13.125v1.5c0 .621-.504 1.125-1.125 1.125M6 13.125C6 12.504 5.496 12 4.875 12m-1.5 0h1.5m-1.5 0c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125M19.125 12h1.5m0 0c.621 0 1.125.504 1.125 1.125v1.5c0 .621-.504 1.125-1.125 1.125m-17.25 0h1.5m14.25 0h1.5"></path>
        </svg>
      </div>
    </td>
    <td class="px-4 py-3 text-xs">
      <StatusPill :watched-status="request?.userWatchHistory?.watchedStatus" />
    </td>
    <td class="px-4 py-3 text-sm">
      <div class="flex items-center text-sm">
        <!-- Avatar with inset shadow -->
        <div class="relative mr-3 hidden h-8 w-8 rounded-full md:block">
          <img
            class="h-full w-full rounded-full object-cover"
            :src="request?.mediaRequest.requestedBy.avatar"
            alt=""
            loading="lazy" />
          <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true"></div>
        </div>
        <div class="flex flex-col space-y-1">
          <p class="font-semibold">
            {{ request?.mediaRequest.requestedBy.plexUsername ?? request?.mediaRequest.requestedBy.username }}
          </p>
          <p class="text-xs">
            <span
              :data-user-type="request?.mediaRequest.requestedBy.userType"
              class="rounded-md px-2 py-[2px] text-white">
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

const userType = (userType?: number): 'Local User' | 'Plex User' => (userType == 1 ? 'Plex User' : 'Local User')
// YYYY-MM-DD - dont like it? Make your own app. Just be happy its not my preferred 24H
const getDate = (newDate?: string) => (newDate ? new Date(newDate).toLocaleDateString('en-CA') : 'N/A')
const getTime = (newDate?: string) => (newDate ? new Date(newDate).toLocaleTimeString('en-CA') : 'N/A')
</script>
<style lang="postcss" scoped>
[data-user-type='1'] {
  @apply bg-purple-600;
}

[data-user-type='2'] {
  @apply bg-teal-600;
}
</style>
