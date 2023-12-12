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
    value: false,
    subtitle: 'Wait for show to end before purging anything?',
  },
  {
    title: 'Purge seasons',
    name: 'purgeSeason',
    value: false,
    subtitle: 'Delete seasons as they are watched or wait for show to be watched?',
  },
  {
    title: 'In Progress as latest',
    name: 'inProgressWatched',
    value: false,
    subtitle: 'Use the newest in progress season as watched progress?',
  },
  {
    title: 'Watched as latest',
    name: 'watchedWatched',
    value: false,
    subtitle: 'Use the newest watched season as watched progress?',
  },
  {
    title: 'Purge period',
    name: 'purgePeriod',
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
