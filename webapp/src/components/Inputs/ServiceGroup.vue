<template>
  <div class="flex flex-col gap-2">
    <label class="mr-2 flex flex-col text-sm" :for="name">
      <span class="text-gray-700 dark:text-gray-400">
        {{ label }}
        <span v-if="required" class="text-red-600">*</span>
      </span>
    </label>
    <div class="flex grow flex-row place-items-center">
      <slot></slot>
    </div>
    <p v-for="error in errors" :key="error.$uid" class="text-xs text-red-600">{{ error.$message }}</p>
  </div>
</template>
<script lang="ts" setup>
import { ErrorObject } from '@vuelidate/core'
import { PropType } from 'vue'

defineProps({
  label: {
    type: String,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
  required: {
    type: Boolean,
    required: false,
    default: false,
  },
  errors: {
    type: Object as PropType<ErrorObject[]>,
    required: true,
    default: [] as ErrorObject[],
  },
})
</script>
