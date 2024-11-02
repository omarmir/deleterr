<template>
  <div
    class="flex items-center justify-between rounded-lg bg-orange-600 p-4 text-sm font-semibold text-white shadow-md focus:shadow-outline-purple focus:outline-none"
    target="_blank">
    <div class="flex items-center">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="mr-2 size-5">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z" />
      </svg>

      <slot></slot>
    </div>
  </div>
</template>
