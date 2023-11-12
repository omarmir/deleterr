<template>
  <transition appear enter-from-class="translate-y-[10%] opacity-0" enter-active-class="transition duration-150">
    <div class="mb-6" @mouseenter="mouseEnter()" @mouseleave="mouseLeave()">
      <div
        class="w-full space-y-3 overflow-hidden overflow-y-auto rounded-lg bg-white shadow-lg ring-0 dark:bg-gray-800">
        <div class="p-4 text-sm font-medium">
          <div class="flex justify-between">
            <p
              class="flex items-center text-sm font-semibold"
              :class="toast.danger ? 'text-red-600' : 'text-gray-700 dark:text-gray-400'">
              {{ toast.title }}
            </p>
            <button class="p-1" @click="closeToast(toast.id)">
              <svg
                class="h-5 w-5 text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200"
                fill="none"
                stroke="currentColor"
                stroke-width="1.5"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
                aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          <p class="pt-2 text-gray-500 dark:text-gray-400">{{ toast.message }}</p>
        </div>
        <div
          class="h-1 transition-all"
          :class="toast.danger ? 'bg-rejectedtext' : 'bg-accent'"
          :style="{ width: width + '%' }"></div>
      </div>
    </div>
  </transition>
</template>
<script lang="ts" setup>
import { useToast, Toast } from '~/composables/useToast'
import { Ref, ref } from 'vue'
const { closeToast } = useToast()

const props = defineProps({
  toast: { required: true, type: Object as () => Toast },
})

const width: Ref<number> = ref(100)

/**
 * x 1000 since its in seconds then /100 since its going to do this a 100 times (100% to 0%) so in total x 10
 */
const duration: number = props.toast.duration * 10 + 3

const countDownWidth = () => {
  width.value--
  if (width.value === 0) {
    clearInterval(widthInterval)
    closeToast(props.toast.id)
  }
}

let widthInterval = setInterval(countDownWidth, duration)

const mouseEnter = () => clearInterval(widthInterval)

const mouseLeave = () => {
  widthInterval = setInterval(countDownWidth, duration)
}
</script>
