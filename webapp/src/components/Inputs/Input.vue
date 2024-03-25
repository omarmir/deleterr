<template>
  <input
    :id="name"
    v-model="model"
    class="w-full appearance-none rounded-md border bg-white py-2 pl-2 text-sm focus:border-purple-400 focus:shadow-outline-purple focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-300 dark:focus:shadow-outline-gray"
    :placeholder="placeholder"
    :type="type"
    :name="name" />
</template>
<script lang="ts" setup>
import { PropType } from 'vue'

defineProps({
  placeholder: String,
  required: {
    type: Boolean,
    required: false,
    default: false,
  },
  type: {
    type: String as PropType<'number' | 'text' | 'password'>,
    required: true,
  },
  name: {
    type: String,
    required: true,
  },
})

const model = defineModel()
</script>
