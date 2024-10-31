<template>
  <div class="grid gap-8 lg:grid-cols-2">
    <Service service-type="overseerr" :service="store.services?.overseerr" logo="/assets/images/overseerr.svg" />
    <Service service-type="tautulli" :service="store.services?.tautulli" logo="/assets/images/tautulli.svg">Tautulli
    </Service>
    <Service service-type="sonarr" :service="store.services?.sonarr" logo="/assets/images/sonarr.svg">Sonarr</Service>
    <Service service-type="radarr" :service="store.services?.radarr" logo="/assets/images/radarr.svg">Radarr</Service>
  </div>
</template>
<script lang="ts" setup async>
import Service from '~/components/Service.vue'
import { useServiceStore } from '~/stores/services.store'
const store = useServiceStore()

await store.getServices()
</script>
