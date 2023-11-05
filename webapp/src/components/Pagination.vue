<template>
  <!-- TODO: REFACTOR the buttons not in SFC-->
  <div class="text-outlinetext flex justify-center text-xs text-white">
    <ul v-if="pageCount > 10">
      <!--<li class="rounded-l-lg">-->
      <li :class="{ disabled: selectedPage === 1 }">
        <button :disabled="selectedPage === 1" @click="setPage(selectedPage - 1)">
          <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5"></path>
          </svg>
        </button>
      </li>
      <li v-for="item in 3" :key="item" :class="{ current: selectedPage === item }">
        <PaginationButton :current-page="item" :selected-page="selectedPage" @change-page="setPage" />
      </li>
      <li :class="{ hidden: isLeftTruncateHidden }" class="disabled">
        <button disabled>
          <span>...</span>
        </button>
      </li>
      <li v-for="item in pageCount - 6" :key="item" :class="getPageClass(item + 3)">
        <PaginationButton :current-page="item + 3" :selected-page="selectedPage" @change-page="setPage" />
      </li>
      <li :class="{ hidden: isRightTruncateHidden }" class="disabled">
        <button disabled>
          <span>...</span>
        </button>
      </li>
      <li v-for="item in 3" :key="item" :class="{ current: selectedPage === pageCount - (3 - item) }">
        <PaginationButton :current-page="pageCount - (3 - item)" :selected-page="selectedPage" @change-page="setPage" />
      </li>
      <!--<li class="border-r rounded-r-lg">-->
      <li :class="{ disabled: selectedPage === pageCount }">
        <button :disabled="selectedPage === pageCount" @click="setPage(selectedPage + 1)">
          <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5"></path>
          </svg>
        </button>
      </li>
    </ul>
    <!-- TODO: This needs to be redone-->
    <ul v-else>
      <li v-for="item in pageCount" :key="item" :class="[item == selectedPage ? 'current' : '']">
        <PaginationButton :current-page="item" :selected-page="selectedPage" @change-page="setPage" />
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import PaginationButton from '~/components/PaginationButton.vue'

const props = defineProps({
  pageCount: { type: Number, required: true },
  selectedPage: { type: Number, required: true },
})

const emit = defineEmits<{
  (e: 'changePage', page: number): void
}>()

const getPageClass = (item: number): string => {
  if (item === props.selectedPage) return 'current'

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

<style lang="postcss" scoped>
ul {
  @apply flex list-none flex-row flex-wrap text-center;
}

button {
  @apply h-8 w-8;
}

li {
  @apply h-8 w-8 rounded-full;

  &:not(.disabled):not(.current):hover {
    @apply bg-purple-950;
  }

  &.current {
    @apply bg-purple-300 font-medium text-white;
  }
}
</style>
