<template>
  <li :class="{ hidden: isTruncateHidden }" class="disabled">
    <button disabled>
      <span>...</span>
    </button>
  </li>
</template>
<script lang="ts" setup>
defineProps({
  isTruncateHidden: { type: Boolean, required: true },
})
</script>
