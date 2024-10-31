<template>
  <div class="shadow-xs w-full overflow-hidden">
    <div v-if="error" class="text-red-400">
      {{ error.toString() }}
    </div>
    <div v-if="!error && data?.success" class="w-full overflow-x-auto">
      <ul>
        <li
          v-for="request in data.data?.requests"
          :key="request.mediaRequest.id"
          class="border-t border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-900">
          <div class="flex w-full flex-col gap-3 p-3 lg:flex-row lg:gap-2">
            <div class="flex min-h-16 basis-full space-x-4 lg:basis-4/12" :data-id="request.mediaRequest.id">
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
              <!--
              <Actions
                :is-exempt="store.isMediaExempted(request?.mediaRequest?.id)"
                :external-id="request?.mediaRequest.media.externalServiceId"
                :deletion-callback="() => deleteMedia(request?.mediaRequest.id, request?.mediaRequest.media.mediaType)"
                :exemption-callback="() => toggleExempt(request?.mediaRequest.id)" />-->
            </div>
          </div>
        </li>
      </ul>
      <!--
      <PaginationWrapper
        :take="store.tableState.take"
        :filtered-requests="store.filteredRequests"
        :selected-page="store.currentPage"
        :page-count="store.pageCount ?? 1"
        @change-page="store.changePage" />-->
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
import { APIResponse, MediaType, MovieDeletionRequest, RequestStatusWithRecordInfo } from '~/@types/deleterr'
import { useDebounce, useFetch } from '@vueuse/core'
import { Ref, ref } from 'vue'
import { useUrl } from 'vue-useurl'

const search: Ref<null | string> = ref(null)

const { url } = useUrl({
  path: '/api/v1/json/requests',
  queryParams: {
    sortBy: 'requestedDate',
    isDescending: true,
    take: 5,
    skip: undefined,
    search: useDebounce(search, 500),
  },
})

const { error, data } = await useFetch<APIResponse<RequestStatusWithRecordInfo>>(url, { refetch: true })

const isTV = (mediaType?: MediaType): boolean => {
  return (mediaType ?? 'movie') == 'tv' ? true : false
}
</script>
