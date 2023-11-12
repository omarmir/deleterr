import { nanoid } from 'nanoid'
import { Ref, ref } from 'vue'
export type Toast = {
    id: string
    title: string
    message: string
    duration: number
    danger: boolean
}

const currentToasts: Ref<Toast[]> = ref([])

export const useToast = () => {
    /**
     * Publish a toast to the screen
     * @param title The title of the toast message
     * @param message The message of the toast screen
     * @param duration How many seconds did you want this to be on the screen for
     * @param danger is this bad news? This will change the colour on the item
     */
    const publishToast = (title: string, message: string, duration: number, danger: boolean = false) => {
        currentToasts.value.push({ id: nanoid(), title, message, duration, danger })
    }

    const closeToast = (id: string) => {
        const toastIndex: number = currentToasts.value.findIndex((toast) => toast.id === id)
        currentToasts.value.splice(toastIndex, 1)
    }

    return {
        publishToast,
        closeToast,
        currentToasts,
    }
}