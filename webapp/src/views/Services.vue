<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Services</h2>
  <div class="grid gap-8 lg:grid-cols-2">
    <Service logo="/overseerr.svg">Overseerr</Service>
    <Service logo="/tautulli.png">Tautulli</Service>
    <Service logo="/sonarr.svg">Sonarr</Service>
    <Service logo="/radarr.svg">Radarr</Service>
  </div>
  <div class="grid lg:grid-cols-2 lg:space-x-6"></div>
</template>
<script lang="ts" setup>
import Service from '~/components/Service.vue'
</script>
