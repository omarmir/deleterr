<template>
  <label class="flex items-center dark:text-gray-400">
    <input
      type="checkbox"
      class="form-checkbox dark:focus:shadow-outline-gray text-purple-600 focus:border-purple-400 focus:shadow-outline-purple focus:outline-none" />
    <span class="ml-2">
      <slot></slot>
    </span>
  </label>
</template>
