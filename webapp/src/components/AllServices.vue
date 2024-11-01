<template>
  <div class="grid gap-8 lg:grid-cols-2" v-if="!error && data?.success">
    <Service service-type="overseerr" :service-info="data?.data?.overseerr" logo="/assets/images/overseerr.svg" />
    <Service service-type="tautulli" :service-info="data?.data?.tautulli" logo="/assets/images/tautulli.svg">
      Tautulli
    </Service>
    <Service service-type="sonarr" :service-info="data?.data?.sonarr" logo="/assets/images/sonarr.svg">Sonarr</Service>
    <Service service-type="radarr" :service-info="data?.data?.radarr" logo="/assets/images/radarr.svg">Radarr</Service>
  </div>
  <Error :error :api-result="data"></Error>
</template>
<script lang="ts" setup async>
import { useFetch } from '@vueuse/core'
import { APIResponse, ServiceInfo, Services } from '~/@types/deleterr'
import Service from '~/components/Service.vue'

const { error, data } = await useFetch('/api/v1/json/service/get', {
  immediate: true,
})
  .get()
  .json<APIResponse<Record<Services, ServiceInfo>>>()
</script>
