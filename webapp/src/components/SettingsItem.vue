<template>
  <div>
    <div class="grid grid-cols-1 items-start lg:grid-cols-3">
      <label class="flex basis-full flex-col justify-start gap-2 dark:text-gray-400 lg:basis-1/3" :for="option.name">
        <p class="text-sm font-bold">{{ option.title }}</p>
        <p class="pr-2 text-sm">
          {{ option.subtitle }}
        </p>
      </label>
      <div class="col-span-2 flex flex-row place-items-center gap-4">
        <input
          v-if="option.type == 'boolean'"
          :name="option.name"
          type="checkbox"
          class="peer relative h-6 w-6 shrink-0 appearance-none rounded-md border-2 border-purple-600 bg-white checked:border-0 checked:bg-purple-600 focus:shadow-outline-purple focus:outline-none dark:focus:shadow-outline-gray" />
        <div v-else-if="option.type == 'array'" class="flex flex-row place-items-center">
          <select
            class="block appearance-none rounded-md border py-2 pl-2 pr-8 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray">
            <option v-for="item in option.value" :key="item.name" :value="item.value">{{ item.label }}</option>
          </select>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            strokeWidth="{1.5}"
            stroke="currentColor"
            class="pointer-events-none -ml-6 h-4 w-4 dark:text-gray-300">
            <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 8.25l-7.5 7.5-7.5-7.5" />
          </svg>
        </div>
        <svg
          class="pointer-events-none absolute mt-1 hidden h-5 w-5 text-white peer-checked:block"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 28"
          fill="none"
          stroke="currentColor"
          stroke-width="4"
          stroke-linecap="round"
          stroke-linejoin="round">
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
        <input
          v-if="option.additionalDetail"
          :aria-label="option.additionalDetail?.label"
          :name="option.additionalDetail.name"
          class="block rounded-md border p-2 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
          :placeholder="option.additionalDetail?.label"
          type="number"
          label="" />
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { PropType } from 'vue'
import { SettingsOption } from '~/@types/settings'

const props = defineProps({
  option: { type: Object as PropType<SettingsOption>, required: true },
  disabled: {
    type: Boolean,
    required: false,
    default: false,
  },
})

const setValue = (event: Event): void => {
  const newVal: string = (event.target as HTMLInputElement).value
  if (!props.disabled) emit('update:modelValue', newVal)
}

const emit = defineEmits(['update:modelValue'])
</script>
