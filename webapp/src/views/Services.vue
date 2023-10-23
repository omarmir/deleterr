<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">
    Services
  </h2>
  <div class="flex flex-row lg:space-x-6">
    <Service logo="/overseerr.svg">Overseerr</Service>
    <Service logo="/tautulli.png">Tautulli</Service>
  </div>
  <div class="flex flex-row lg:space-x-6">
    <Service logo="/sonarr.svg">Sonarr</Service>
    <Service logo="/radarr.svg">Radarr</Service>
  </div>
</template>
<script lang="ts" setup>
import Service from '~/components/Service.vue'
</script>
