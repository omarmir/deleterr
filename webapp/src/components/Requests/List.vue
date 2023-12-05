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
          <div class="flex w-full flex-col space-y-2 p-3 lg:flex-row">
            <div class="flex h-16 basis-full space-x-4 lg:basis-4/12" :data-id="request.mediaRequest.id">
              <RequestsListItemsThumb
                :images="request?.mediaInfo?.images"
                :title="request?.mediaInfo?.title"
                :media-type="request?.mediaRequest.media.mediaType"></RequestsListItemsThumb>
              <RequestsListItemsNameRelease
                :title="request?.mediaInfo?.title"
                :ended="request?.mediaInfo?.ended"
                :release-date="request?.mediaInfo?.releaseDate"></RequestsListItemsNameRelease>
            </div>
            <div class="flex basis-full overflow-hidden lg:basis-3/12">
              <RequestsListItemsSeasons
                :media-type="request?.mediaRequest.media.mediaType"
                :season-status="request?.seasonStatus"></RequestsListItemsSeasons>
            </div>
            <div class="flex basis-4/12 flex-row space-x-2">
              <div class="flex basis-2/5 flex-col space-y-4 lg:basis-1/4">
                <RequestsListItemsTypeIcon
                  :media-type="request?.mediaRequest.media.mediaType"></RequestsListItemsTypeIcon>
                <StatusPill :watched-status="request.watched" />
              </div>
              <div class="flex basis-3/5 flex-row space-y-1 lg:basis-3/4 lg:flex-col">
                <p class="text-sm font-bold text-gray-700 dark:text-gray-400">Requested:</p>
                <div class="flex flex-row space-x-2">
                  <RequestsListItemsRequested
                    :created-at="request?.mediaRequest.createdAt"></RequestsListItemsRequested>
                  <span class="text-sm text-gray-700 dark:text-gray-400">by</span>
                  <RequestsListItemsUser :media-request="request?.mediaRequest"></RequestsListItemsUser>
                </div>
              </div>
            </div>
            <div class="flex basis-1/12">
              <Actions
                :is-exempt="store.isMediaExempted(request?.mediaRequest?.id)"
                :exemption-button-state="store.actionStates['exemption_' + request.mediaRequest.id]"
                :deletion-button-state="store.actionStates['delete_' + request.mediaRequest.id]"
                :external-id="request?.mediaRequest.media.externalServiceId"
                @delete-media="deleteMedia(request?.mediaRequest.id, request?.mediaRequest.media.mediaType)"
                @toggle-exempt="toggleExempt(request?.mediaRequest.id, request?.mediaRequest.media.tmdbId)" />
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
import Actions from '~/components/Actions.vue'
import { useRequestsStore } from '~/stores/requests.store'
import { MediaType, SingleMediaExeption } from '~/@types/deleterr'

const store = useRequestsStore()

await store.getRequests()

const toggleExempt = async (requestId?: number, tmdbId?: number) => {
  if (requestId && tmdbId) {
    let exemption: SingleMediaExeption = [requestId, tmdbId]
    await store.toggleMediaExemption(exemption)
  }
}

const deleteMedia = (requestId?: number, mediaType?: MediaType) => {
  if (requestId && mediaType) {
    if (mediaType == 'movie') {
      store.deleteMovieFile(requestId)
    }
  }
}
</script>
