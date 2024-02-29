<template>
  <button
    :type="isSubmit ? 'submit' : 'button'"
    :class="
      isOutlined
        ? 'border-purple-600 text-purple-600 active:bg-purple-600 dark:text-white'
        : 'border-transparent bg-purple-600 text-white active:bg-purple-600'
    "
    class="flex flex-row border px-4 py-2 text-sm font-medium leading-5 transition-colors duration-150 hover:bg-purple-700 focus:shadow-outline-purple focus:outline-none">
    <slot name="icons"></slot>
    <slot />
  </button>
</template>
<script lang="ts" setup>
defineProps({
  isSubmit: { type: Boolean, required: false, default: false },
  isOutlined: { type: Boolean, required: false, default: false },
})
</script>
