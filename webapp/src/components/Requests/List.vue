<template>
  <div class="shadow-xs w-full overflow-hidden">
    <div v-if="store.error" class="text-red-400">
      {{ store.error.toString() }}
    </div>
    <div v-if="!store.error" class="w-full overflow-x-auto">
      <ul class="space-y-2">
        <li
          v-for="request in store.requests"
          :key="request.mediaRequest.id"
          class="rounded-lg bg-gray-100 dark:bg-gray-800">
          <div class="flex w-full flex-row p-3">
            <div class="flex h-16 basis-4/12 space-x-4" :data-id="request.mediaRequest.id">
              <RequestsListItemsThumb :images="request?.mediaInfo?.images"></RequestsListItemsThumb>
              <RequestsListItemsNameRelease
                :title="request?.mediaInfo?.title"
                :release-date="request?.mediaInfo?.releaseDate"></RequestsListItemsNameRelease>
            </div>
            <div class="flex basis-4/12">
              <RequestsListItemsSeasons
                :media-type="request?.mediaRequest.media.mediaType"
                :season-status="request?.seasonStatus"></RequestsListItemsSeasons>
            </div>
            <div class="flex basis-1/12 flex-col">
              <div class="flex flex-col space-y-2">
                <RequestsListItemsTypeIcon
                  :media-type="request?.mediaRequest.media.mediaType"></RequestsListItemsTypeIcon>
                <StatusPill :watched-status="request.watched" />
              </div>
            </div>
            <div class="flex basis-3/12 flex-col space-y-1">
              <p class="text-sm font-bold text-gray-700 dark:text-gray-400">Requested:</p>
              <div class="flex flex-row space-x-2">
                <RequestsListItemsRequested :created-at="request?.mediaRequest.createdAt"></RequestsListItemsRequested>
                <p class="text-sm text-gray-700 dark:text-gray-400">by</p>
                <RequestsListItemsUser :media-request="request?.mediaRequest"></RequestsListItemsUser>
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
import StatusPill from '~/components/Requests/ListItems/StatusPill.vue'
import RequestsListItemsThumb from '~/components/Requests/ListItems/Thumb.vue'
import RequestsListItemsNameRelease from '~/components/Requests/ListItems/NameRelease.vue'
import RequestsListItemsRequested from '~/components/Requests/ListItems/Requested.vue'
import RequestsListItemsTypeIcon from '~/components/Requests/ListItems/TypeIcon.vue'
import RequestsListItemsSeasons from '~/components/Requests/ListItems/Seasons.vue'
import RequestsListItemsUser from '~/components/Requests/ListItems/User.vue'
//import Actions from '~/components/Actions.vue'
import { useRequestsStore } from '~/stores/requests.store'

const store = useRequestsStore()

await store.getRequests()
</script>
