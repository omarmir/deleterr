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
          class="border-t border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-900">
          <div class="flex w-full flex-col gap-3 p-3 lg:flex-row lg:gap-2">
            <div class="min-h-16 flex basis-full space-x-4 lg:basis-4/12" :data-id="request.mediaRequest.id">
              <RequestsListItemsThumb
                :images="request?.mediaInfo?.images"
                :title="request?.mediaInfo?.title"
                :media-type="request?.mediaRequest.media.mediaType"></RequestsListItemsThumb>
              <RequestsListItemsNameRelease
                :title="request?.mediaInfo?.title"
                :ended="request?.mediaInfo?.ended"
                :release-date="request?.mediaInfo?.releaseDate"></RequestsListItemsNameRelease>
            </div>
            <div class="flex basis-full place-items-center lg:basis-2/12">
              <StatusPill :watched-status="request.watched" />
            </div>
            <div class="flex basis-5/12 flex-row flex-wrap gap-2 lg:self-center">
              <div
                v-if="isTV(request?.mediaRequest.media.mediaType)"
                class="flex basis-full flex-row flex-wrap place-items-center gap-2">
                <RequestsListTVIcon :media-type="request?.mediaRequest.media.mediaType"></RequestsListTVIcon>
                <RequestsListItemsSeasons
                  :media-type="request?.mediaRequest.media.mediaType"
                  :season-status="request?.seasonStatus"></RequestsListItemsSeasons>
              </div>
              <RequestsListMovieIcon v-else :media-type="request?.mediaRequest.media.mediaType"></RequestsListMovieIcon>
              <span class="text-sm font-bold text-gray-700 dark:text-gray-400">Requested:</span>
              <RequestsListItemsRequested :created-at="request?.mediaRequest.createdAt"></RequestsListItemsRequested>
              <span class="text-sm text-gray-700 dark:text-gray-400">by</span>
              <RequestsListItemsUser :media-request="request?.mediaRequest"></RequestsListItemsUser>
            </div>
            <div class="flex basis-full lg:basis-1/12">
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
import RequestsListTVIcon from '~/components/Requests/ListItems/TVIcon.vue'
import RequestsListMovieIcon from '~/components/Requests/ListItems/MovieIcon.vue'
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

const isTV = (mediaType?: MediaType): boolean => {
  return (mediaType ?? 'movie') == 'tv' ? true : false
}
</script>
