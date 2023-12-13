<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Settings</h2>
  <Suspense>
    <div class="flex flex-col space-y-5">
      <div>
        <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">Password</h4>
        <ContentCard>
          <form class="flex flex-col space-y-3" @submit.prevent="">
            <InputsInputRightButton
              v-model="store.newPassword"
              type="password"
              :required="true"
              label="Change password"
              button-label="Update"
              placeholder="New password"></InputsInputRightButton>
          </form>
        </ContentCard>
      </div>
      <div>
        <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">Deletion Rules</h4>
        <ContentCard>
          <form class="flex flex-col space-y-6" @submit.prevent="">
            <div class="flex flex-col gap-3">
              <h5 class="text-md font-semibold text-gray-600 dark:text-gray-300">TV Series</h5>
              <SettingsItem v-for="option in tvOptions" :key="option.name" :option="option"></SettingsItem>
            </div>
            <div class="flex flex-col gap-3">
              <h5 class="text-md font-semibold text-gray-600 dark:text-gray-300">Movies</h5>
              <SettingsItem v-for="option in movieOptions" :key="option.name" :option="option"></SettingsItem>
            </div>
            <div class="flex justify-end">
              <ButtonsStatused :is-submit="true" :is-outlined="false" class="rounded-lg">Save changes</ButtonsStatused>
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
import ButtonsStatused from '~/components/Buttons/Statused.vue'
import SettingsItem from '~/components/SettingsItem.vue'
import { SettingsOption } from '~/@types/settings'
import { useSettingsStore } from '~/stores/settings.store'

const tvOptions: Array<SettingsOption> = [
  {
    title: 'Wait for show to end',
    name: 'showEnd',
    type: 'boolean',
    value: false,
    subtitle: 'Wait for show to end before purging anything?',
  },
  {
    title: 'Purge seasons',
    name: 'purgeSeason',
    type: 'boolean',
    value: false,
    subtitle: 'Delete seasons as they are watched or wait for show to be watched?',
  },
  {
    title: 'Watched marker',
    name: 'watchedMarker',
    type: 'array',
    value: [
      { name: 'inProgress', label: 'Latest in progress', value: 'inProgress' },
      { name: 'watched', label: 'Latest watched', value: 'watched' },
      { name: 'tautulli', label: 'Tautulli only', value: 'tautulli' },
    ],
    subtitle:
      'You can use the latest in progress or watched season as a watch progress marker or you can use only tautulli',
  },
  {
    title: 'Purge period',
    name: 'purgePeriod',
    type: 'boolean',
    value: false,
    subtitle: 'Delete shows after a set number of days after the last episode has been download?',
    additionalDetail: {
      name: 'period',
      label: 'Number of days',
    },
  },
]

const movieOptions: Array<SettingsOption> = [
  {
    title: 'Purge period',
    name: 'purgePeriod',
    type: 'boolean',
    value: false,
    subtitle: 'Delete movies after a set number of days after the movie has been download?',
    additionalDetail: {
      name: 'period',
      label: 'Number of days',
    },
  },
]

const store = useSettingsStore()
</script>
