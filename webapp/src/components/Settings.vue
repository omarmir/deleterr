<template>
  <div>
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">Deletion Rules</h4>
    <ContentCard>
      <form class="flex flex-col space-y-4" @submit.prevent v-if="!error && data?.success">
        <h5 class="text-md font-semibold text-gray-600 dark:text-gray-300">TV Series</h5>
        <div class="flex max-w-3xl flex-col gap-4">
          <SettingsServiceGroup
            :errors="v$.tvPurgeMarker.$errors"
            :required="true"
            name="tvPurgeMarker"
            label="Purge marker">
            <InputsSelect
              v-model="settings.tvPurgeMarker"
              label="Purge marker"
              :required="true"
              name="tvPurgeMarker"
              :options="[
                { label: 'Watched', value: 'watched' },
                { label: 'Time', value: 'time' },
                { label: 'Both', value: 'both' },
              ]"></InputsSelect>
            <template #subtitle>
              Delete after the requestor has watched the item or after a specified time has elapsed or whichever comes
              first
            </template>
          </SettingsServiceGroup>
          <SettingsServiceGroup
            v-if="settings.tvPurgeMarker !== 'time'"
            :errors="v$.tvWatchedMarker.$errors"
            :required="true"
            name="watchedMarker"
            label="Watched marker">
            <InputsSelect
              v-model="settings.tvWatchedMarker"
              label="Watched marker"
              name="watchedMarker"
              :required="true"
              :options="[
                { label: 'Latest in progress', value: 'inProgress' },
                { label: 'Latest watched', value: 'watched' },
              ]" />
            <template #subtitle>
              You can use the latest in progress or watched season as a watch progress marker
            </template>
          </SettingsServiceGroup>
          <SettingsServiceGroup
            v-if="settings.tvPurgeMarker !== 'watched'"
            :required="true"
            name="tvPurgeDelay"
            :errors="v$.tvPurgeDelay.$errors"
            label="Purge delay">
            <InputsInput
              v-model="settings.tvPurgeDelay"
              name="tvPurgeDelay"
              type="number"
              :required="true"
              label="Purge delay" />
            <template #subtitle>
              Delete shows after a set number of days after the last episode has been download
            </template>
          </SettingsServiceGroup>
          <SettingsServiceGroup
            :errors="v$.tvPurgeStrategy.$errors"
            :required="true"
            name="tvPurgeStrategy"
            label="Purge strategy">
            <InputsSelect
              v-model="settings.tvPurgeStrategy"
              label="Purge strategy"
              name="tvPurgeStrategy"
              :required="true"
              :options="[
                { label: 'Season', value: 'season' },
                { label: 'Show', value: 'show' },
              ]" />
            <template #subtitle>Delete seasons as they are watched or wait for show to be watched</template>
          </SettingsServiceGroup>
          <h5 class="text-md mt-4 font-semibold text-gray-600 dark:text-gray-300">Movies</h5>
          <SettingsServiceGroup
            :errors="v$.moviePurgeMarker.$errors"
            :required="true"
            name="moviePurgeMarker"
            label="Purge marker">
            <InputsSelect
              v-model="settings.moviePurgeMarker"
              label="Purge marker"
              :required="true"
              name="moviePurgeMarker"
              :options="[
                { label: 'Watched', value: 'watched' },
                { label: 'Time', value: 'time' },
                { label: 'Both', value: 'both' },
              ]"></InputsSelect>
            <template #subtitle>
              Delete after the requestor has watched the item or after a specified time has elapsed regardless of watch
              status or whichever comes first
            </template>
          </SettingsServiceGroup>
          <SettingsServiceGroup
            v-if="settings.moviePurgeMarker !== 'watched'"
            :required="true"
            :errors="v$.moviePurgeDelay.$errors"
            name="moviePurgeDelay"
            label="Purge delay">
            <InputsInput
              v-model="settings.moviePurgeDelay"
              name="moviePurgeDelay"
              type="number"
              :required="true"
              label="Purge delay" />
            <template #subtitle>
              Delete shows after a set number of days after the last episode has been download
            </template>
          </SettingsServiceGroup>
          <div class="flex justify-end">
            <ButtonsStatus @click="saveSettings" :is-outlined="false" :provided-operation-state="savingState">
              Save Changes
            </ButtonsStatus>
          </div>
        </div>
      </form>
      <Error :error="error" :api-result="data"></Error>
    </ContentCard>
  </div>
</template>
<script lang="ts" setup async>
import InputsSelect from '~/components/Inputs/Select.vue'
import InputsInput from '~/components/Inputs/Input.vue'
import SettingsServiceGroup from '~/components/Inputs/SettingsGroup.vue'
import ContentCard from '~/components/ContentCard.vue'
import useVuelidate from '@vuelidate/core'
import Error from '~/components/Error.vue'
import ButtonsStatus from '~/components/Buttons/Status.vue'
import { required, numeric, minValue, requiredIf } from '@vuelidate/validators'
import { Settings } from '~/@types/deleterr'
import { APIResponse } from '~/@types/deleterr'
import { useFetch } from '@vueuse/core'
import { useSettings } from '~/composables/useSettings'

const { error, data } = await useFetch('/api/v1/json/settings/get', {
  immediate: true,
})
  .get()
  .json<APIResponse<Settings>>()

const { save, settings, savingState } = useSettings(data.value?.data)

const rules = {
  tvPurgeMarker: { required },
  tvWatchedMarker: {
    requiredIf: requiredIf(() => settings.tvPurgeMarker !== 'time'),
  },
  tvPurgeDelay: {
    requiredIf: requiredIf(() => settings.tvPurgeMarker !== 'watched'),
    minValue: minValue(1),
    numeric: numeric,
  },
  tvPurgeStrategy: { required },
  moviePurgeMarker: { required },
  moviePurgeDelay: {
    requiredIf: requiredIf(() => settings.moviePurgeMarker !== 'watched'),
    minValue: minValue(1),
    numeric: numeric,
  },
}

const v$ = useVuelidate(rules, settings)

const saveSettings = async () => {
  const validation = await v$.value.$validate()
  if (validation) save()
}
</script>
