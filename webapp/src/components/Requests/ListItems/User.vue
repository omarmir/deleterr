<template>
  <div class="flex items-center text-sm text-gray-700 dark:text-gray-400">
    <!-- Avatar with inset shadow -->
    <div class="relative mr-3 hidden h-8 w-8 rounded-full md:block">
      <img
        class="h-full w-full rounded-full object-cover"
        :src="mediaRequest?.requestedBy.avatar"
        alt=""
        loading="lazy" />
      <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true"></div>
    </div>
    <div class="flex flex-col space-y-1">
      <p class="font-semibold">
        {{ mediaRequest?.requestedBy.plexUsername ?? mediaRequest?.requestedBy.username }}
      </p>
      <p class="text-xs">
        <span :data-user-type="mediaRequest?.requestedBy.userType" class="rounded-md px-2 py-[2px] text-white">
          {{ userType(mediaRequest?.requestedBy.userType) }}
        </span>
      </p>
    </div>
  </div>
</template>
<script setup lang="ts">
import { PropType } from 'vue'
import { MediaRequest } from '~/@types/deleterr.ts'
defineProps({
  mediaRequest: { type: Object as PropType<MediaRequest>, required: false },
})
const userType = (userType?: number): 'Local User' | 'Plex User' => (userType == 1 ? 'Plex User' : 'Local User')
</script>
