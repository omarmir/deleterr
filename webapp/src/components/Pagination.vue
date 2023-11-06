<template>
  <div
    class="grid border-t bg-gray-50 px-4 py-3 text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 sm:grid-cols-9">
    <span class="col-span-3 flex items-center">Showing {{ take * selectedPage + 1 }}-{{ Math.min(take * selectedPage + 5, allRequests) }} of {{ allRequests }}</span>
    <span class="col-span-2"></span>
    <!-- Pagination -->
    <span class="col-span-4 mt-2 flex sm:mt-auto sm:justify-end">
      <nav aria-label="Table navigation">
        <ul v-if="pageCount > 10" class="inline-flex items-center">
          <li>
            <button class="rounded-md rounded-l-lg px-3 py-1 focus:shadow-outline-purple focus:outline-none" aria-label="Previous">
              <svg aria-hidden="true" class="h-4 w-4 fill-current" viewBox="0 0 20 20">
                <path
                  d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                  clip-rule="evenodd"
                  fill-rule="evenodd"></path>
              </svg>
            </button>
          </li>
          <li v-for="item in 3" :key="item">
            <button
              :class="[item == selectedPage + 1 ? 'current' : '']"
              class="rounded-md rounded-l-lg px-3 py-1 hover:bg-purple-500 hover:bg-opacity-20 focus:shadow-outline-purple focus:outline-none"
              @click="setPage(item - 1)">
              {{ item }}
            </button>
          </li>
          <li :class="{ hidden: isLeftTruncateHidden }" class="disabled">
            <button disabled>
              <span>...</span>
            </button>
          </li>
          <li v-for="item in pageCount - 6" :key="item" :class="getPageClass(item + 3)">
            <button
              :class="[item == selectedPage - 2 ? 'current' : '']"
              class="rounded-md rounded-l-lg px-3 py-1 hover:bg-purple-500 hover:bg-opacity-20 focus:shadow-outline-purple focus:outline-none"
              @click="setPage(item + 2)">
              {{ item + 3 }}
            </button>
          </li>
          <li :class="{ hidden: isRightTruncateHidden }" class="disabled">
            <button disabled>
              <span>...</span>
            </button>
          </li>
          <li v-for="item in 3" :key="item">
            <button
              :class="[item == selectedPage - pageCount + 4 ? 'current' : '']"
              class="rounded-md rounded-l-lg px-3 py-1 hover:bg-purple-500 hover:bg-opacity-20 focus:shadow-outline-purple focus:outline-none"
              @click="setPage(pageCount - (4 - item))">
              {{ pageCount - (3 - item) }}
            </button>
          </li>
          <li>
            <button class="rounded-md rounded-r-lg px-3 py-1 focus:shadow-outline-purple focus:outline-none" aria-label="Next">
              <svg class="h-4 w-4 fill-current" aria-hidden="true" viewBox="0 0 20 20">
                <path
                  d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                  clip-rule="evenodd"
                  fill-rule="evenodd"></path>
              </svg>
            </button>
          </li>
        </ul>
        <ul v-else class="inline-flex items-center">
          <li v-for="item in pageCount" :key="item">
            <button
              :class="[item == selectedPage ? 'current' : '']"
              class="rounded-md rounded-l-lg px-3 py-1 hover:bg-purple-500 hover:bg-opacity-20 focus:shadow-outline-purple focus:outline-none"
              :current-page="item"
              :selected-page="selectedPage"
              @change-page="setPage">
              {{ item }}
            </button>
          </li>
        </ul>
      </nav>
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps({
  pageCount: { type: Number, required: true },
  selectedPage: { type: Number, required: true },
  take: { type: Number, required: true },
  allRequests: { type: Number, required: false, default: 0 },
})

const emit = defineEmits<{
  (e: 'changePage', page: number): void
}>()

const getPageClass = (item: number): string => {
  if (item === props.selectedPage) return ''

  // You always want to show 15 items

  /** If the selected page is 7 or less then you dont get both the truncates
   *  So you show the first 11 items + 1 truncate + 3 last digits = 15
   * */
  if (props.selectedPage < 8 && item < 12) return ''

  /**
   * If you are on the last 7 pages then you don't get both the truncates
   * So you show the last 11 values
   * Since the first 3 always show and then the truncate shows - 11 + 3 + 1 = 15
   */
  if (props.selectedPage > props.pageCount - 7 && item > props.pageCount - 11) return ''

  // Show 3 on either side of the selected page IF its not in the first 7 or last 7 pages
  if (Math.abs(item - props.selectedPage) < 4) return ''

  // Otherwise just hide the pages
  return 'hidden'
}

const isLeftTruncateHidden = computed(() => {
  return props.selectedPage < 8
})

const isRightTruncateHidden = computed(() => {
  return props.selectedPage > props.pageCount - 7
})

const setPage = (item: number): void => {
  emit('changePage', item)
}
</script>

<style scoped lang="postcss">
.current {
  @apply border border-r-0 border-purple-600 bg-purple-600 text-white transition-colors duration-150;
}
</style>
