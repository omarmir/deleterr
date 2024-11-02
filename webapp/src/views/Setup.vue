<template>
  <BlankPage>
    <h1 class="mb-4 text-xl font-semibold text-gray-700 dark:text-gray-200">Add Credentials</h1>
    <form class="space-y-4" @submit.prevent>
      <InputsInput
        v-model="authUser.username"
        type="text"
        name="username"
        :required="true"
        label="Username"
        placeholder="Username" />
      <InputsInput
        v-model="authUser.password"
        name="password"
        type="password"
        label="Password"
        placeholder="Password" />
      <ButtonsStatus :provided-operation-state="setupState" :is-outlined="false" :is-submit="true">
        Add User
      </ButtonsStatus>
    </form>
  </BlankPage>
</template>
<script setup lang="ts">
import InputsInput from '~/components/Inputs/Input.vue'
import ButtonsStatus from '~/components/Buttons/Status.vue'
import BlankPage from '~/components/BlankPage.vue'
import { APIResponse, AuthenticationUser } from '~/@types/deleterr'
import { reactive, ref } from 'vue'
import { useVuelidate } from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import { useAuthStore } from '~/stores/auth.store'
import { useRouter } from 'vue-router'
import { OperationState } from '~/@types/common'

const store = useAuthStore()
const router = useRouter()

const authUser: AuthenticationUser = reactive({
  username: '',
  password: '',
})

const rules = {
  username: { required },
  password: { required },
}

const v$ = useVuelidate(rules, authUser as any)

const setupState = ref(OperationState.hidden)

const submitForm = async (): Promise<APIResponse<boolean | undefined> | undefined> => {
  setupState.value = OperationState.loading

  const result = await v$.value.$validate()

  if (result) {
    const result = await store.addInitialUser(authUser)
    if (result.success) {
      router.push(router.currentRoute.value.redirectedFrom?.fullPath ?? '/login')
      setupState.value = OperationState.success
    } else {
      setupState.value = OperationState.failure
    }
    return result
  }
}
</script>
