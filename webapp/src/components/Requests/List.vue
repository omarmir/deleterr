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
            <RequestsListItemsThumb :poster-path="request?.mediaInfo?.posterPath"></RequestsListItemsThumb>
            <RequestsListItemsNameRelease
              :title="request?.mediaInfo?.title"
              :release-date="request?.mediaInfo?.releaseDate"></RequestsListItemsNameRelease>
            <RequestsListItemsRequested :created-at="request?.mediaRequest.createdAt"></RequestsListItemsRequested>
            <RequestsListItemsTypeIcon :media-type="request?.mediaRequest.media.mediaType"></RequestsListItemsTypeIcon>
            <RequestsListItemsSeasons
              :media-type="request?.mediaRequest.media.mediaType"
              :season-status="request?.seasonStatus"></RequestsListItemsSeasons>
            <StatusPill :watched-status="request.watched" />
            <RequestsListItemsUser :media-request="request?.mediaRequest"></RequestsListItemsUser>
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
<style lang="postcss" scoped>
[data-user-type='1'] {
  @apply bg-purple-600;
}

[data-user-type='2'] {
  @apply bg-teal-600;
}
</style>
