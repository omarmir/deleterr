<template>
  <div class="grid gap-8 lg:grid-cols-2">
    <Service
      service-type="overseerr"
      :service-status="store.serviceStatus.overseerr"
      :service="store.services?.overseerr"
      logo="/overseerr.svg"
      @save="store.saveService"
      @test="store.testService" />
    <Service
      service-type="tautulli"
      :service-status="store.serviceStatus.tautulli"
      :service="store.services?.tautulli"
      logo="/tautulli.svg"
      @save="store.saveService"
      @test="store.testService">
      Tautulli
    </Service>
    <Service
      service-type="sonarr"
      :service-status="store.serviceStatus.sonarr"
      :service="store.services?.sonarr"
      logo="/sonarr.svg"
      @save="store.saveService"
      @test="store.testService">
      Sonarr
    </Service>
    <Service
      service-type="radarr"
      :service-status="store.serviceStatus.radarr"
      :service="store.services?.radarr"
      logo="/radarr.svg"
      @save="store.saveService"
      @test="store.testService">
      Radarr
    </Service>
  </div>
</template>
<script lang="ts" setup async>
import Service from '~/components/Service.vue'
import { useServiceStore } from '~/stores/services.store'
const store = useServiceStore()

await store.getServices()
</script>
