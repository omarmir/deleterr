<template>
  <div>
    <h4 class="mb-4 text-lg font-semibold text-gray-600 dark:text-gray-300">Password</h4>
    <ContentCard>
      <form class="flex max-w-3xl flex-col space-y-3" @submit.prevent>
        <InputsServiceGroup :required="true" name="newPassword" label="Password" :errors="v$.newPassword.$errors">
          <InputsInputRightButton
            @click="updatePassword"
            v-model="newPassword.newPassword"
            :provided-operation-state="savePasswordState"
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
</template>
<script lang="ts" setup>
import ContentCard from '~/components/ContentCard.vue'
import InputsInputRightButton from '~/components/Inputs/InputRightButton.vue'
import InputsServiceGroup from '~/components/Inputs/ServiceGroup.vue'
import { required } from '@vuelidate/validators'
import useVuelidate from '@vuelidate/core'
import { nextTick, ref, Ref } from 'vue'
import { useAuthStore } from '~/stores/auth.store'
import { OperationState } from '~/@types/common'

const store = useAuthStore()

const newPassword: Ref<{ newPassword: string | undefined }> = ref({ newPassword: undefined })

const rules = {
  newPassword: { required },
}

const v$ = useVuelidate(rules, newPassword)

const savePasswordState = ref(OperationState.hidden)

const updatePassword = async () => {
  savePasswordState.value = OperationState.loading
  const validation = await v$.value.$validate()
  if (validation && newPassword.value.newPassword) {
    const updateResp = await store.updatePassword(newPassword.value.newPassword)
    if (updateResp?.success) {
      newPassword.value.newPassword = ''
      await nextTick()
      v$.value.$reset()
      savePasswordState.value = OperationState.success
    } else {
      savePasswordState.value = OperationState.failure
    }
  }
}
</script>
