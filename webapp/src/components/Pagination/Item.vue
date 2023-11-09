<template>
  <button
    :class="[item == selectedPage ? 'current' : '']"
    class="rounded-md rounded-l-lg px-3 py-1 hover:bg-purple-500 hover:bg-opacity-20 focus:shadow-outline-purple focus:outline-none"
    @click="$emit('setPage')">
    <slot></slot>
  </button>
</template>
<script lang="ts" setup>
defineProps({
  item: { type: Number, required: true },
  selectedPage: { type: Number, required: true },
})

defineEmits(['setPage'])
</script>
<style scoped lang="postcss">
.current {
  @apply border border-r-0 border-purple-600 bg-purple-600 text-white transition-colors duration-150;
}
</style>
