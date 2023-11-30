<template>
  <img class="h-16 w-11" :src="'https://image.tmdb.org/t/p/w600_and_h900_bestv2' + posterPath" alt="" loading="lazy" />
</template>
<script setup lang="ts">
defineProps({
  posterPath: { type: String, required: false },
})
</script>
