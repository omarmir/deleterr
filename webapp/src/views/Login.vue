<template>
  <BlankPage>
    <h1 class="mb-4 text-xl font-semibold text-gray-700 dark:text-gray-200">Login</h1>
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
      <ButtonsStatused :callback="submitForm" :is-outlined="false" class="rounded-lg" :is-submit="true">
        Login
      </ButtonsStatused>
    </form>
  </BlankPage>
</template>
<script setup lang="ts">
import InputsInput from '~/components/Inputs/Input.vue'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
import BlankPage from '~/components/BlankPage.vue'
import { APIResponse, AuthenticationUser } from '~/@types/deleterr'
import { reactive } from 'vue'
import { useVuelidate } from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import { useAuthStore } from '~/stores/auth.store'
import { useRouter } from 'vue-router'

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

const submitForm = async (): Promise<APIResponse<String> | undefined> => {
  const result = await v$.value.$validate()

  if (result) {
    const result = await store.login(authUser)
    if (result.success) {
      console.log()
      router.push(router.currentRoute.value.redirectedFrom?.fullPath ?? '/')
    }
    return result
  }
}
</script>
