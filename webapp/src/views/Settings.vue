<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Settings</h2>
  <Suspense>
    <div class="flex flex-col space-y-5">
      <CTA action-link="https://github.com/omarmir/deleterr">
        <span>This is ALPHA software. Features are unstable. Please report any issues on GitHub!</span>
        <template #action>
          <span>GitHub &RightArrow;</span>
        </template>
      </CTA>
      <div>
        <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">Password</h4>
        <ContentCard>
          <form class="flex max-w-3xl flex-col space-y-3" @submit.prevent="">
            <InputsServiceGroup :required="true" name="password" label="Password">
              <InputsInputRightButton
                v-model="store.newPassword"
                :callback="store.updatePassword"
                name="password"
                type="password"
                :required="true"
                label="Change password"
                button-label="Update"
                placeholder="New password"></InputsInputRightButton>
            </InputsServiceGroup>
          </form>
        </ContentCard>
      </div>
      <div>
        <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">Deletion Rules</h4>
        <ContentCard>
          <form class="flex flex-col space-y-4" @submit.prevent="">
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
                  Delete after the requestor has watched the item or after a specified time has elapsed or whichever
                  comes first
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
                  Delete after the requestor has watched the item or after a specified time has elapsed regardless of
                  watch status or whichever comes first
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
              <div class="flex justify-end">
                <ButtonsStatused :callback="saveSettings" :is-submit="true" :is-outlined="false" class="rounded-lg">
                  Save changes
                </ButtonsStatused>
              </div>
            </div>
          </form>
        </ContentCard>
      </div>
    </div>
    <template #fallback>
      <PageLoading>
        <template #message>Loading</template>
      </PageLoading>
    </template>
  </Suspense>
</template>
<script lang="ts" setup async>
import PageLoading from '~/components/PageLoading.vue'
import ContentCard from '~/components/ContentCard.vue'
import InputsInputRightButton from '~/components/Inputs/InputRightButton.vue'
import InputsSelect from '~/components/Inputs/Select.vue'
import InputsInput from '~/components/Inputs/Input.vue'
import { useSettingsStore } from '~/stores/settings.store'
import InputsServiceGroup from '~/components/Inputs/ServiceGroup.vue'
import SettingsServiceGroup from '~/components/Inputs/SettingsGroup.vue'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
import CTA from '~/components/CTA.vue'
import { APIResponse, Settings } from '~/@types/deleterr'
import { useVuelidate } from '@vuelidate/core'
import { numeric, required, minValue } from '@vuelidate/validators'

const store = useSettingsStore()

const saveSettings = async (): Promise<APIResponse<Settings> | undefined> => {
  const rules = {
    tvMoviePurgeMarker: { required },
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
  }
}
</script>
