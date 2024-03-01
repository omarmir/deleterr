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
              <InputsSelect
                label="Deletion strategy"
                :required="true"
                :options="[
                  { name: 'Watched', value: 'watched' },
                  { name: 'Time', value: 'time' },
                  { name: 'Both', value: 'both' },
                ]"></InputsSelect>
              <InputsCheckbox label="Wait for show to end?" :required="true"></InputsCheckbox>
            </div>
            <div class="flex flex-col gap-3">
              <h5 class="text-md font-semibold text-gray-600 dark:text-gray-300">Movies</h5>
            </div>
            <div class="flex justify-end">
              <!--<ButtonsStatused :is-submit="true" :is-outlined="false" class="rounded-lg">Save changes</ButtonsStatused>-->
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
import InputsCheckbox from '~/components/Inputs/Checkbox.vue'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
import { useSettingsStore } from '~/stores/settings.store'
import CTA from '~/components/CTA.vue'

const store = useSettingsStore()

/**
 * Deletion strategy: Watched / Time / Both
Wait for show to end

if Watched:
Watched marker

Time
Purge period

Both
Watche marker
Purge period

 */
</script>
