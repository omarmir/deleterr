<template>
  <BlankPage>
    <h1 class="mb-4 text-xl font-semibold text-gray-700 dark:text-gray-200">Login</h1>
    <form class="space-y-4" @submit="submitForm">
      <InputsServiceGroup :required="true" name="username" label="Username" :errors="v$.username.$errors">
        <InputsInput
          v-model="authUser.username"
          type="text"
          name="username"
          :required="true"
          label="Username"
          v-on-key-stroke:Enter,v="submitForm"
          placeholder="Username" />
      </InputsServiceGroup>
      <InputsServiceGroup :required="true" name="password" label="password" :errors="v$.password.$errors">
        <InputsInput
          v-model="authUser.password"
          name="password"
          type="password"
          label="Password"
          v-on-key-stroke:Enter,v="submitForm"
          placeholder="Password" />
      </InputsServiceGroup>
      <ButtonsStatus @click="submitForm" :provided-operation-state="loginState" :is-outlined="false">
        Login
      </ButtonsStatus>
    </form>
  </BlankPage>
</template>
<script setup lang="ts">
import InputsInput from '~/components/Inputs/Input.vue'
import ButtonsStatus from '~/components/Buttons/Status.vue'
import BlankPage from '~/components/BlankPage.vue'
import InputsServiceGroup from '~/components/Inputs/ServiceGroup.vue'
import { APIResponse, AuthenticationUser } from '~/@types/deleterr'
import { reactive, ref } from 'vue'
import { useVuelidate } from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import { useAuthStore } from '~/stores/auth.store'
import { useRouter } from 'vue-router'
import { OperationState } from '~/@types/common'
import { vOnKeyStroke } from '@vueuse/components'

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

const loginState = ref(OperationState.hidden)

const submitForm = async (): Promise<APIResponse<String> | undefined> => {
  loginState.value = OperationState.loading
  const result = await v$.value.$validate()

  if (result) {
    const result = await store.login(authUser)
    if (result.success) {
      loginState.value = OperationState.success
      router.push(router.currentRoute.value.redirectedFrom?.fullPath ?? '/')
    } else {
      loginState.value = OperationState.failure
    }
    return result
  } else {
    loginState.value = OperationState.failure
  }
}
</script>
