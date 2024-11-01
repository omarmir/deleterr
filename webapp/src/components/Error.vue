<template>
  <div v-if="!apiResult?.success" class="text-red-400">
    {{ apiResult?.error_msg }}
  </div>
  <div v-else-if="error" class="text-red-400">
    {{ error.toString() }}
  </div>
</template>
<script setup lang="ts">
import { APIResponse } from '~/@types/deleterr'

const { error, apiResult } = defineProps<{ error: any; apiResult: APIResponse<any> | null }>()
</script>
