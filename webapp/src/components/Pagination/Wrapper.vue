<template>
  <div
    class="grid border-t px-4 py-3 text-xs font-semibold uppercase tracking-wide text-gray-500 dark:border-gray-700 dark:text-gray-400 sm:grid-cols-9">
    <PaginationPageCount :take="take" :selected-page="selectedPage" :filtered-requests="filteredRequests" />
    <span class="col-span-2"></span>
    <!-- Pagination -->
    <span class="col-span-4 mt-2 flex sm:mt-auto sm:justify-end">
      <nav aria-label="Table navigation">
        <ul v-if="pageCount > 10" class="inline-flex items-center">
          <PaginationPrevious @click="$emit('decrement')" />
          <li v-for="item in 3" :key="item">
            <PaginationItem :item="item" :selected-page="selectedPage + 1" @set-page="setPage(item - 1)">
              {{ item }}
            </PaginationItem>
          </li>
          <PaginationTruncate :is-truncate-hidden="isLeftTruncateHidden" />
          <li v-for="item in pageCount - 6" :key="item" :class="{ hidden: isHidden(item + 3) }">
            <PaginationItem :item="item" :selected-page="selectedPage - 2" @set-page="setPage(item + 2)">
              {{ item + 3 }}
            </PaginationItem>
          </li>
          <PaginationTruncate :is-truncate-hidden="isRightTruncateHidden" />
          <li v-for="item in 3" :key="item">
            <PaginationItem
              :item="item"
              :selected-page="selectedPage - pageCount + 4"
              @set-page="setPage(pageCount - (4 - item))">
              {{ pageCount - (3 - item) }}
            </PaginationItem>
          </li>
          <PaginationNext @click="$emit('increment')" />
        </ul>
        <ul v-else class="inline-flex items-center">
          <li v-for="item in pageCount" :key="item">
            <PaginationItem :item="item" :selected-page="selectedPage + 1" @set-page="setPage(item - 1)">
              {{ item }}
            </PaginationItem>
          </li>
        </ul>
      </nav>
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import PaginationPageCount from '~/components/Pagination/PageCount.vue'
import PaginationPrevious from '~/components/Pagination/Previous.vue'
import PaginationItem from '~/components/Pagination/Item.vue'
import PaginationNext from '~/components/Pagination/Next.vue'
import PaginationTruncate from '~/components/Pagination/Truncation.vue'

const props = defineProps({
  pageCount: { type: Number, required: true },
  selectedPage: { type: Number, required: true },
  take: { type: Number, required: true },
  filteredRequests: { type: Number, required: true, default: 0 },
})

const emit = defineEmits<{
  (e: 'changePage', page: number): void
  (e: 'increment'): void
  (e: 'decrement'): void
}>()

const isHidden = (item: number): boolean => {
  const { selectedPage, pageCount } = props

  return (
    item !== selectedPage &&
    // You always want to show 15 items

    /** If the selected page is 7 or less then you dont get both the truncates
     *  So you show the first 11 items + 1 truncate + 3 last digits = 15
     * */
    !(selectedPage < 8 && item < 12) &&
    /**
     * If you are on the last 7 pages then you don't get both the truncates
     * So you show the last 11 values
     * Since the first 3 always show and then the truncate shows - 11 + 3 + 1 = 15
     */
    !(selectedPage > pageCount - 7 && item > pageCount - 11) &&
    // Show 3 on either side of the selected page IF its not in the first 7 or last 7 pages

    Math.abs(item - selectedPage) >= 4
  )
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
