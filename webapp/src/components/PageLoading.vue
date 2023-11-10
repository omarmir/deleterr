<template>
  <div class="flex flex-col place-items-center gap-5 text-gray-500 dark:text-gray-400">
    <div>
      <svg class="h-10 w-10" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
        <path
          fill="none"
          stroke="currentColor"
          stroke-dasharray="15"
          stroke-dashoffset="15"
          stroke-linecap="round"
          stroke-width="2"
          d="M12 3C16.9706 3 21 7.02944 21 12">
          <animate fill="freeze" attributeName="stroke-dashoffset" dur="0.3s" values="15;0" />
          <animateTransform
            attributeName="transform"
            dur="1.5s"
            repeatCount="indefinite"
            type="rotate"
            values="0 12 12;360 12 12" />
        </path>
      </svg>
    </div>
    <div>
      <slot name="message"></slot>
    </div>
    <small class="text-center">
      <slot name="subtitle"></slot>
    </small>
  </div>
</template>
