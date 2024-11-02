<template>
  <ButtonsBase :is-outlined class="rounded-lg" :is-submit>
    <template #icons>
      <span v-if="operationState !== OperationState.hidden" class="pr-2">
        <svg
          v-if="operationState === OperationState.loading"
          class="h-5 w-5"
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24">
          <path
            fill="none"
            stroke="currentColor"
            stroke-dasharray="15"
            stroke-dashoffset="15"
            stroke-linecap="round"
            stroke-width="2"
            d="M12 3C16.9706 3 21 7.02944 21 12">
            <animate fill="freeze" attributeName="stroke-dashoffset" dur="0.3s" values="15;0" />
            <animateTransform
              attributeName="transform"
              dur="1.5s"
              repeatCount="indefinite"
              type="rotate"
              values="0 12 12;360 12 12" />
          </path>
        </svg>
        <svg
          v-else-if="operationState === OperationState.success"
          class="h-5 w-5 text-green-500"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
          aria-hidden="true">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <svg
          v-else-if="operationState === OperationState.failure"
          class="h-5 w-5 text-red-500"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
          aria-hidden="true">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"></path>
        </svg>
      </span>
    </template>
    <slot />
  </ButtonsBase>
</template>
<script lang="ts" setup>
import { defineProps, ref, watch } from 'vue'
import { OperationState } from '~/@types/common'
import ButtonsBase from '~/components/Buttons/Base.vue'

const {
  providedOperationState,
  isSubmit = false,
  isOutlined = true,
} = defineProps<{
  providedOperationState: OperationState
  isSubmit?: boolean
  isOutlined?: boolean
}>()

const operationState = ref(providedOperationState)

watch(
  () => providedOperationState,
  () => {
    operationState.value = providedOperationState
    setTimeout(() => {
      operationState.value = OperationState.hidden
    }, 5000)
  }
)
</script>
