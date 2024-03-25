<template>
  <div class="flex min-h-screen items-center bg-gray-50 p-6 dark:bg-gray-900">
    <div class="mx-auto h-full max-w-xl flex-1 overflow-hidden">
      <div class="mb-4 space-x-2 font-lato text-2xl font-bold text-gray-800 dark:text-gray-200">
        <div class="flex flex-row place-content-center space-x-2">
          <svg
            class="w-6"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            version="1.1"
            x="-20px"
            y="0px"
            viewBox="8 0 80 100"
            enable-background="new 0 0 100 100"
            xml:space="preserve">
            <path
              fill="currentColor"
              d="M69.657,87.453c-22.385,4.014-39.425-0.06-39.425-0.06l0.812,5.461c0,0,16.689,3.856,37.993,0.113  L69.657,87.453z" />
            <path
              fill="currentColor"
              d="M63.02,9.855C52.068,1.266,41.726,7.227,38.312,9.693c-11.913,0.902-19.366,2.909-19.366,2.909l1.275,8.48  c0.183-0.049,26.843-7.177,59.448,0l1.275-8.481C74.717,11.231,68.7,10.362,63.02,9.855z M21.898,18.663l-0.707-4.15  c0,0,3.798-1.153,9.828-1.686l0.606,4.194C25.704,17.516,21.945,18.649,21.898,18.663z" />
            <path
              fill="currentColor"
              d="M21.317,25.008l8.741,58.156c0,0,17.216,4.12,39.773,0l8.741-58.155  C47.17,18.096,21.494,24.961,21.317,25.008z M36.267,73.456l-5.904-38.401c1.357-0.19,4.228-0.55,8.227-0.837l1.632,39.648  C38.927,73.755,37.608,73.456,36.267,73.456z M52.335,74.663c-1.431,0.019-2.948,0.012-4.531-0.026l-1.511-42.45  c2.298-0.059,4.778-0.074,7.406-0.022L52.335,74.663z M63.281,73.668c-1.061,0.115-2.383,0.238-3.928,0.347l1.642-39.885  c2.659,0.168,5.419,0.409,8.252,0.739L63.281,73.668z" />
          </svg>
          <span>deleterr</span>
        </div>
      </div>
      <div class="flex flex-col overflow-y-auto rounded-lg bg-white shadow-xl dark:bg-gray-800 md:flex-row">
        <div class="flex items-center justify-center p-6 sm:p-12 md:w-full">
          <div class="w-full">
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
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import InputsInput from '~/components/Inputs/Input.vue'
import ButtonsStatused from '~/components/Buttons/Statused.vue'
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
