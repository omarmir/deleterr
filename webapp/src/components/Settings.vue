<template>
  <div>
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">Deletion Rules</h4>
    <ContentCard>
      <form class="flex flex-col space-y-4" @submit.prevent>
        <h5 class="text-md font-semibold text-gray-600 dark:text-gray-300">TV Series</h5>
        <div class="flex max-w-3xl flex-col gap-4">
          <SettingsServiceGroup :required="true" name="tvPurgeMarker" label="Purge marker">
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
          <SettingsServiceGroup :required="true" name="tvPurgeStrategy" label="Purge strategy">
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
          <SettingsServiceGroup :required="true" name="moviePurgeMarker" label="Purge marker">
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
          <p v-for="error in errors" :key="error.$uid" class="text-xs text-red-600">
            {{ error.$property + ' - ' + error.$message }}
          </p>
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
import { required, numeric, minValue } from '@vuelidate/validators'
import { Settings } from '~/@types/deleterr'
import { APIResponse } from '~/@types/deleterr'
import { useSettingsStore } from '~/stores/settings.store'
import { ErrorObject } from '@vuelidate/core'
import { Ref, ref } from 'vue'

const store = useSettingsStore()

const errors: Ref<ErrorObject[]> = ref([])

await store.getSettings()

const saveSettings = async (): Promise<APIResponse<Settings> | undefined> => {
  const rules = {
    tvPurgeMarker: { required },
    tvWatchedMarker: {},
    tvPurgeDelay: {},
    tvPurgeStrategy: { required },
    moviePurgeMarker: { required },
    moviePurgeDelay: {},
  }

  switch (store.settings?.tvPurgeMarker) {
    case 'watched':
      rules.tvWatchedMarker = { required }
      break
    case 'time':
      rules.tvPurgeDelay = { required, numeric, minValue: minValue(1) }
      break
    default:
      rules.tvWatchedMarker = { required }
      rules.tvPurgeDelay = { required, numeric, minValue: minValue(1) }
  }

  if (store.settings?.moviePurgeMarker !== 'watched')
    rules.moviePurgeDelay = { required, numeric, minValue: minValue(1) }

  const v$ = useVuelidate(rules, store.settings as any)

  const validation = await v$.value.$validate()
  if (validation) {
    return store.saveSettings()
  } else {
    errors.value = v$.value.$errors
  }
}
</script>
