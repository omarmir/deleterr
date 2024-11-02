<template>
  <div class="shadow-xs w-full overflow-hidden">
    <div v-if="!error && requests?.success && requests.data" class="w-full overflow-x-auto">
      <ul>
        <li
          v-for="request in requests.data.requests"
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
              <DeleteSeries v-if="isTV(request?.mediaRequest.media.mediaType)"></DeleteSeries>
              <ActionsDelete
                v-else
                @delete="execute()"
                :external-id="request?.mediaRequest.media.externalServiceId"
                :request-id="request.mediaRequest.id"></ActionsDelete>
              <ActionsExemption
                v-if="!exemptionError && exemptions?.success"
                @exemption-added="addExemption"
                @exemption-removed="removeExemption"
                :exemptions="exemptions.data ?? []"
                :request-id="request.mediaRequest.id"></ActionsExemption>
            </div>
          </div>
        </li>
      </ul>
      <PaginationWrapper
        :take="query.take"
        :filtered-requests="requests.data?.filteredRequests"
        :selected-page="currentPage"
        :page-count="Math.ceil(requests.data.filteredRequests / query.take)"
        @change-page="changePage" />
    </div>
    <Error :error :api-result="requests"></Error>
    <Modal :open="false"></Modal>
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
import ActionsDelete from '~/components/Actions/Delete.vue'
import ActionsExemption from '~/components/Actions/Exemption.vue'
import Error from '~/components/Error.vue'
import DeleteSeries from '../Actions/DeleteSeries.vue'
import { APIResponse, MediaExemptions, MediaType, RequestStatusWithRecordInfo } from '~/@types/deleterr'
import { useFetch } from '@vueuse/core'
import { useListQuery } from '~/composables/useListQuery'
import { inject, ref, Ref } from 'vue'

const search = inject<Ref<string>>('search') ?? ref(null)

const { url, query, currentPage, changePage } = useListQuery({ search, take: 5 })

const {
  error,
  data: requests,
  execute,
} = await useFetch(url, {
  refetch: true,
  immediate: true,
  timeout: 30000,
})
  .get()
  .json<APIResponse<RequestStatusWithRecordInfo>>()

const { error: exemptionError, data: exemptions } = await useFetch('/api/v1/json/request/exemptions/get', {
  immediate: true,
})
  .get()
  .json<APIResponse<MediaExemptions>>()

const removeExemption = (id: number) => {
  if (exemptions.value?.success) exemptions.value.data = exemptions.value.data?.filter((exemp) => exemp !== id)
}

const addExemption = (id: number) => {
  if (exemptions.value?.success) exemptions.value.data?.push(id)
}

const isTV = (mediaType?: MediaType): boolean => {
  return (mediaType ?? 'movie') == 'tv' ? true : false
}
</script>
