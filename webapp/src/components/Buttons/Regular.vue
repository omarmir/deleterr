<template>
  <button
    :type="isSubmit ? 'submit' : 'button'"
    class="rounded-lg border border-transparent bg-purple-600 px-4 py-2 text-sm font-medium leading-5 text-white transition-colors duration-150 hover:bg-purple-700 focus:shadow-outline-purple focus:outline-none active:bg-purple-600">
    <slot />
  </button>
</template>
<script lang="ts" setup>
defineProps({
  isSubmit: { type: Boolean, required: false, default: false },
})
</script>
