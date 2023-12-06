<template>
  <div class="flex shrink-0 text-sm text-gray-700 dark:text-gray-400">{{ timeAgoSimple(createdAt) }}</div>
</template>
<script setup lang="ts">
defineProps({
  createdAt: { type: String, required: false },
})

function timeAgoSimple(date?: string): string {
  if (!date) return 'Unknown'

  const currentDate = new Date()
  const inputDate = new Date(date)

  const timeDifference = currentDate.getTime() - inputDate.getTime()
  const days = Math.floor(timeDifference / (24 * 60 * 60 * 1000))

  if (days > 0) {
    return `${days} day${days > 1 ? 's' : ''} ago`
  } else {
    return 'less than a day ago'
  }
}
</script>
