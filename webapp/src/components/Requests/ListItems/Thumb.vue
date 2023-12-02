<template>
  <img class="h-16 w-11" src="#" />
</template>
<script setup lang="ts">
import { Image } from '~/@types/deleterr'

defineProps({
  images: { type: Array<Image>, required: false },
})
</script>
