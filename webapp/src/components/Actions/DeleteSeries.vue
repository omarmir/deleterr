<template>
  <button
    class="flex items-center justify-between rounded-lg px-2 py-2 text-sm font-medium leading-5 text-red-500 focus:shadow-outline-gray focus:outline-none"
    aria-label="Delete"
    @click="openModal">
    <svg class="h-5 w-5" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20">
      <path
        fill-rule="evenodd"
        d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
        clip-rule="evenodd"></path>
    </svg>
  </button>
  <div
    v-show="isOpen"
    class="fixed inset-0 z-30 flex items-end bg-black bg-opacity-50 sm:items-center sm:justify-center">
    <div
      v-show="isOpen"
      @keydown.escape="closeModal"
      class="w-full overflow-hidden rounded-t-lg bg-white px-6 py-4 dark:bg-gray-800 sm:m-4 sm:max-w-xl sm:rounded-lg"
      role="dialog"
      id="modal">
      <!-- Modal body -->
      <div class="mb-6 mt-4">
        <!-- Modal title -->
        <p class="mb-2 text-lg font-semibold text-gray-700 dark:text-gray-300">Delete Series</p>
        <!-- Modal description -->
        <p class="text-sm text-gray-700 dark:text-gray-400">
          You can either delete the whole series or just the watched seasons.
        </p>
      </div>
      <footer
        class="-mx-6 -mb-4 flex flex-col items-center justify-between bg-gray-50 px-6 py-3 dark:bg-gray-800 sm:flex-row sm:space-x-6 sm:space-y-0">
        <ButtonsBase class="rounded-lg" :is-outlined="true" @click="closeModal">Cancel</ButtonsBase>
        <div class="flex flex-row space-x-4">
          <ButtonsStatus
            :is-outlined="false"
            :provided-operation-state="deletingWatchedState"
            @click="deleteWatched()"
            :auto-icon-shift="false">
            <template #icon>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-5">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" />
                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
              </svg>
            </template>
            <span>Delete watched</span>
          </ButtonsStatus>
        </div>
      </footer>
    </div>
  </div>
</template>

<script setup lang="ts">
import ButtonsStatus from '~/components/Buttons/Status.vue'
import ButtonsBase from '~/components/Buttons/Base.vue'

import { ref, watch } from 'vue'
import { useDeleteSeries } from '~/composables/useDeleteSeries'
import { OperationState } from '~/@types/common'
const { requestId } = defineProps<{ requestId: number }>()

const isOpen = ref(false)

function closeModal() {
  isOpen.value = false
}
function openModal() {
  isOpen.value = true
}

const { deleteWatched, deletingWatchedState } = useDeleteSeries(requestId)

const emits = defineEmits(['deleted'])

watch(deletingWatchedState, () => {
  if (deletingWatchedState.value === OperationState.success) emits('deleted')
})
</script>
