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
          <form class="flex flex-col space-y-3" @submit.prevent="">
            <h5 class="text-md font-semibold text-gray-600 dark:text-gray-300">TV Series</h5>
            <div>
              <InputsRadioButtons
                v-for="option in tvOptions"
                :key="option.options[0].name"
                :label="option.label"
                :options="option.options"></InputsRadioButtons>
            </div>
            <InputsInput type="number" :is-horizontal="true" placeholder="Number of days" label=""></InputsInput>
            <h5 class="text-md font-semibold text-gray-600 dark:text-gray-300">Movies</h5>
            <div>
              <InputsRadioButtons
                v-for="option in movieOptions"
                :key="option.options[0].name"
                :label="option.label"
                :options="option.options"></InputsRadioButtons>
            </div>
            <InputsInput type="number" placeholder="Number of days" :is-horizontal="true" label=""></InputsInput>
            <div class="flex justify-end">
              <ButtonsStatused :is-submit="true" :is-outlined="false" class="rounded-lg">Save</ButtonsStatused>
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
import InputsInput from '~/components/Inputs/Input.vue'
import InputsInputRightButton from '~/components/Inputs/InputRightButton.vue'
import InputsRadioButtons from '~/components/Inputs/RadioButtons.vue'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
//import InputsCheckbox from '~/components/Inputs/Checkbox.vue'
import { SettingsOption } from '~/@types/settings'
import { useSettingsStore } from '~/stores/settings.store'

const tvOptions: Array<{ label: string; options: SettingsOption[] }> = [
  {
    label: 'Wait for show to end?',
    options: [
      { name: 'wait_show_end', value: true, label: 'Yes' },
      { name: 'wait_show_end', value: false, label: 'No' },
    ],
  },
  {
    label: 'Delete seasons as they are watched or wait for show to be watched?',
    options: [
      { name: 'delete_by_season', value: true, label: 'Seasons' },
      { name: 'delete_by_season', value: false, label: 'Show' },
    ],
  },
  {
    label: 'Use the newest in progress season as watched progress?',
    options: [
      { name: 'latest_inprogress_is_watched', value: true, label: 'Yes' },
      { name: 'latest_inprogress_is_watched', value: false, label: 'No' },
    ],
  },
  {
    label: 'Use the newest watched season as watched progress?',
    options: [
      { name: 'latest_watched_is_watched', value: true, label: 'Yes' },
      { name: 'latest_watched_is_watched', value: false, label: 'No' },
    ],
  },
  {
    label: 'Delete shows after a set number of days after the last episode has been download?',
    options: [
      { name: 'delete_after_duration', value: true, label: 'Yes' },
      { name: 'delete_after_duration', value: false, label: 'No' },
    ],
  },
]

const movieOptions: Array<{ label: string; options: SettingsOption[] }> = [
  {
    label: 'Delete movies after a set number of days after download?',
    options: [
      { name: 'delete_after_duration', value: true, label: 'Yes' },
      { name: 'delete_after_duration', value: false, label: 'No' },
    ],
  },
]

const store = useSettingsStore()
</script>
