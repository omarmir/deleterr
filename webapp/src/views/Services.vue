<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Services</h2>
  <div class="grid gap-8 lg:grid-cols-2">
    <Service service="overseerr" logo="/overseerr.svg">Overseerr</Service>
    <Service service="tautulli" logo="/tautulli.png">Tautulli</Service>
    <Service service="sonarr" logo="/sonarr.svg">Sonarr</Service>
    <Service service="radarr" logo="/radarr.svg">Radarr</Service>
  </div>
</template>
<script lang="ts" setup>
import Service from '~/components/Service.vue'
</script>
