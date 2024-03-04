<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Settings</h2>
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
          <InputsServiceGroup :required="true" name="newPassword" label="Password" :errors="v$.newPassword.$errors">
            <InputsInputRightButton
              v-model="store.newPassword.newPassword"
              :callback="updatePassword"
              name="newPassword"
              type="password"
              :required="true"
              label="Change password"
              button-label="Update"
              placeholder="New password"></InputsInputRightButton>
          </InputsServiceGroup>
        </form>
      </ContentCard>
    </div>
    <Suspense>
      <Settings />
      <template #fallback>
        <PageLoading>
          <template #message>Loading</template>
        </PageLoading>
      </template>
    </Suspense>
  </div>
</template>

<script lang="ts" setup>
import PageLoading from '~/components/PageLoading.vue'
import ContentCard from '~/components/ContentCard.vue'
import InputsInputRightButton from '~/components/Inputs/InputRightButton.vue'
import { useSettingsStore } from '~/stores/settings.store'
import InputsServiceGroup from '~/components/Inputs/ServiceGroup.vue'
import CTA from '~/components/CTA.vue'
import Settings from '~/components/Settings.vue'
import { required } from '@vuelidate/validators'
import useVuelidate from '@vuelidate/core'
import { APIResponse } from '~/@types/deleterr'

const store = useSettingsStore()

const rules = {
  newPassword: { required },
}

const v$ = useVuelidate(rules, store.newPassword as any)

const updatePassword = async (): Promise<APIResponse<Boolean> | undefined> => {
  const validation = await v$.value.$validate()
  if (validation) {
    const update_result = await store.updatePassword()
    if (update_result?.success) {
      v$.value.$reset()
    }
    return update_result
  }
}
</script>
