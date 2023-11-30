<template>
  <div class="shadow-xs w-full overflow-hidden">
    <div v-if="store.error" class="text-red-400">
      {{ store.error.toString() }}
    </div>
    <div v-if="!store.error" class="w-full overflow-x-auto">
      <ul class="flex flex-col space-y-2">
        <li
          v-for="request in store.requests"
          :key="request.mediaRequest.id"
          class="rounded-lg bg-gray-100 dark:bg-gray-800">
          <div class="flex flex-row p-2">
            <img
              class="h-16"
              :src="'https://image.tmdb.org/t/p/w600_and_h900_bestv2' + request?.mediaInfo?.posterPath"
              alt=""
              loading="lazy" />
            <div class="flex flex-col space-y-1 text-gray-700 dark:text-gray-400">
              <p class="font-semibold">{{ request?.mediaInfo?.title ?? 'N/A' }}</p>
              <p class="text-xs">Release: {{ request?.mediaInfo?.releaseDate }}</p>
            </div>
            <div class="text-gray-700 dark:text-gray-400">
              <p class="text-sm">{{ getDate(request?.mediaRequest.createdAt) }}</p>
              <p class="text-xs">{{ getTime(request?.mediaRequest.createdAt) }}</p>
            </div>
            <div class="h-5 w-5 text-center text-gray-700 dark:text-gray-400">
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
            <div
              v-if="request?.mediaRequest.media.mediaType === 'tv'"
              class="flex flex-row space-x-2 text-xs text-gray-700 dark:text-gray-400">
              <div v-for="season in request?.seasonStatus" :key="season.seasonNumber">
                {{ season.seasonNumber }} - {{ season.watched }}
              </div>
            </div>
            <div class="px-4 py-3 text-xs">
              <StatusPill :watched-status="request.watched" />
            </div>
            <div class="flex items-center text-sm text-gray-700 dark:text-gray-400">
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
          </div>
        </li>
      </ul>
      <br />
      <PaginationWrapper
        :take="store.tableState.take"
        :filtered-requests="store.filteredRequests"
        :selected-page="store.currentPage"
        :page-count="store.pageCount ?? 1"
        @change-page="store.changePage" />
    </div>
  </div>
</template>
<script lang="ts" setup>
import PaginationWrapper from '~/components/Pagination/Wrapper.vue'
import StatusPill from '~/components/StatusPill.vue'
//import Actions from '~/components/Actions.vue'
import { useRequestsStore } from '~/stores/requests.store'

const store = useRequestsStore()

await store.getRequests()

// YYYY-MM-DD - dont like it? Make your own app. Just be happy its not my preferred 24H
const getDate = (newDate?: string) => (newDate ? new Date(newDate).toLocaleDateString('en-CA') : 'N/A')
const getTime = (newDate?: string) => (newDate ? new Date(newDate).toLocaleTimeString('en-CA') : 'N/A')
const userType = (userType?: number): 'Local User' | 'Plex User' => (userType == 1 ? 'Plex User' : 'Local User')
</script>
<style lang="postcss" scoped>
[data-user-type='1'] {
  @apply bg-purple-600;
}

[data-user-type='2'] {
  @apply bg-teal-600;
}
</style>
