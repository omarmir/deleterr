<template>
  <li class="relative px-6 py-3">
    <router-link
      active-class="current"
      class="inline-flex w-full items-center text-sm font-semibold transition-colors duration-150 hover:text-gray-800 dark:hover:text-gray-200"
      :to="to">
      <span
        class="indicator absolute inset-y-0 left-0 hidden w-1 rounded-br-lg rounded-tr-lg bg-purple-600"
        aria-hidden="true"></span>
      <svg
        class="h-5 w-5"
        aria-hidden="true"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        viewBox="0 0 24 24"
        stroke="currentColor">
        <slot name="svgPath"></slot>
      </svg>
      <span class="ml-4">
        <slot />
      </span>
    </router-link>
  </li>
</template>
<script lang="ts" setup>
defineProps({
  to: { type: String, required: true },
})
</script>
<style lang="postcss" scoped>
/* prettier-ignore */
.current {
    .indicator {
        display: inline;
    }
}
</style>
