<template>
    <div>
        <span :data-status="status" class="rounded-full px-2 py-1 font-semibold capitalize leading-tight">
            {{ status }}
        </span>
    </div>
</template>
<script lang="ts" setup>
const props = defineProps({
    watchedStatus: { required: false, type: Number },
})

enum Status {
    watched = 'watched',
    inprogress = 'inprogress',
    unwatched = 'unwatched',
}

const getWatchedStatus = (watchedStatus: number | undefined): Status => {
    if (watchedStatus) {
        switch (watchedStatus) {
            case 0.5:
                return Status.inprogress
            case 1:
                return Status.watched
            default:
                return Status.unwatched
        }
    } else {
        return Status.unwatched
    }
}

const status: Status = getWatchedStatus(props.watchedStatus)
</script>
<style lang="postcss" scoped>
[data-status='watched'] {
    @apply bg-green-100 text-green-700 dark:bg-green-700 dark:text-green-100;
}
[data-status='inprogress'] {
    @apply bg-orange-100 text-orange-700 dark:bg-orange-600 dark:text-white;
}
[data-status='unwatched'] {
    @apply bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-100;
}
[data-status='unknown'] {
    @apply bg-red-100 text-red-700 dark:bg-red-700 dark:text-red-100;
}
</style>
