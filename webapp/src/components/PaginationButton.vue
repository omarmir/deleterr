<template>
  <button @click="changePage(currentPage)">
    <span>{{ currentPage }}</span>
  </button>
</template>

<script setup lang="ts">
defineProps({
  currentPage: {
    type: Number,
    required: true,
  },
  selectedPage: {
    type: Number,
    required: true,
  },
})

const changePage = (currentPage: number): void => {
  emit('changePage', currentPage)
}

const emit = defineEmits<{
  (e: 'changePage', page: number): void
}>()
</script>
