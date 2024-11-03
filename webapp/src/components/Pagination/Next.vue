<template>
  <li>
    <button
      class="rounded-md rounded-r-lg px-3 py-1 focus:shadow-outline-purple focus:outline-none"
      type="button"
      @click="onClick"
      aria-label="Next">
      <svg class="h-4 w-4 fill-current" aria-hidden="true" viewBox="0 0 20 20">
        <path
          d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
          clip-rule="evenodd"
          fill-rule="evenodd"></path>
      </svg>
    </button>
  </li>
</template>
<script setup lang="ts">
const emits = defineEmits(['click'])

const onClick = () => emits('click')
</script>
