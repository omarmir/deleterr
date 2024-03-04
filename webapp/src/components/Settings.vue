<template>
  <div>
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">Deletion Rules</h4>
    <ContentCard>
      <form class="flex flex-col space-y-4" @submit.prevent>
        <h5 class="text-md font-semibold text-gray-600 dark:text-gray-300">TV Series</h5>
        <div class="flex max-w-3xl flex-col gap-4">
          <SettingsServiceGroup
            :errors="v$.tvPurgeMarker.$errors"
            :required="true"
            name="tvPurgeMarker"
            label="Purge marker">
            <InputsSelect
              v-model="store.settings.tvPurgeMarker"
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
            v-if="store.settings.tvPurgeMarker !== 'time'"
            :errors="v$.tvWatchedMarker.$errors"
            :required="true"
            name="watchedMarker"
            label="Watched marker">
            <InputsSelect
              v-model="store.settings.tvWatchedMarker"
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
            v-if="store.settings.tvPurgeMarker !== 'watched'"
            :required="true"
            name="tvPurgeDelay"
            :errors="v$.tvPurgeDelay.$errors"
            label="Purge delay">
            <InputsInput
              v-model="store.settings.tvPurgeDelay"
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
              v-model="store.settings.tvPurgeStrategy"
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
              v-model="store.settings.moviePurgeMarker"
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
            v-if="store.settings.moviePurgeMarker !== 'watched'"
            :required="true"
            :errors="v$.moviePurgeDelay.$errors"
            name="moviePurgeDelay"
            label="Purge delay">
            <InputsInput
              v-model="store.settings.moviePurgeDelay"
              name="moviePurgeDelay"
              type="number"
              :required="true"
              label="Purge delay" />
            <template #subtitle>
              Delete shows after a set number of days after the last episode has been download
            </template>
          </SettingsServiceGroup>
          <div class="flex justify-end">
            <ButtonsStatused :callback="saveSettings" :is-submit="true" :is-outlined="false" class="rounded-lg">
              Save changes
            </ButtonsStatused>
          </div>
        </div>
      </form>
    </ContentCard>
  </div>
</template>
<script lang="ts" setup async>
import InputsSelect from '~/components/Inputs/Select.vue'
import InputsInput from '~/components/Inputs/Input.vue'
import SettingsServiceGroup from '~/components/Inputs/SettingsGroup.vue'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
import ContentCard from '~/components/ContentCard.vue'
import useVuelidate from '@vuelidate/core'
import { required, numeric, minValue, requiredIf } from '@vuelidate/validators'
import { Settings } from '~/@types/deleterr'
import { APIResponse } from '~/@types/deleterr'
import { useSettingsStore } from '~/stores/settings.store'

const store = useSettingsStore()

await store.getSettings()

const rules = {
  tvPurgeMarker: { required },
  tvWatchedMarker: {
    requiredIf: requiredIf(() => store.settings.tvPurgeMarker !== 'time'),
  },
  tvPurgeDelay: {
    requiredIf: requiredIf(() => store.settings.tvPurgeMarker !== 'watched'),
    minValue: minValue(1),
    numeric: numeric,
  },
  tvPurgeStrategy: { required },
  moviePurgeMarker: { required },
  moviePurgeDelay: {
    requiredIf: requiredIf(() => store.settings.moviePurgeMarker !== 'watched'),
    minValue: minValue(1),
    numeric: numeric,
  },
}

const v$ = useVuelidate(rules, store.settings as any)

const saveSettings = async (): Promise<APIResponse<Settings> | undefined> => {
  const validation = await v$.value.$validate()
  if (validation) {
    return store.saveSettings()
  }
}
</script>
