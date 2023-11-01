<template>
  <div class="grid gap-8 lg:grid-cols-2">
    <Service service-type="overseerr" :service="services?.overseerr" logo="/overseerr.svg" />
    <Service service-type="tautulli" :service="services?.tautulli" logo="/tautulli.svg">Tautulli</Service>
    <Service service-type="sonarr" :service="services?.sonarr" logo="/sonarr.svg">Sonarr</Service>
    <Service service-type="radarr" :service="services?.radarr" logo="/radarr.svg">Radarr</Service>
  </div>
</template>
<script lang="ts" setup async>
import Service from '~/components/Service.vue'
import { ref } from 'vue'
import { APIResponse, ServiceInfo, Services } from '~/@types/deleterr.ts'

const services = ref<Record<Services, ServiceInfo> | undefined>(undefined)

const getServices = async () => {
  try {
    const response = await fetch('http://localhost:8080/api/v1/json/service/get')
    let apiResponse: APIResponse<Record<Services, ServiceInfo>> = await response.json()
    services.value = apiResponse.data
  } catch (error) {
    console.error(error)
  }
}

await getServices()
</script>
