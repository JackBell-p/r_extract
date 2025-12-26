<template>
    <!--
    Toast Container
    - Fixed position at the top-center of the screen.
    - 'pointer-events-none' ensures clicks pass through the container area
      to elements underneath, unless clicking on a specific toast.
    - 'z-50' ensures it stays on top of other elements.
  -->
    <div
        class="fixed top-6 left-0 right-0 flex flex-col items-center pointer-events-none z-50 px-4 gap-3"
    >
        <!-- Animation Wrapper -->
        <TransitionGroup name="toast">
            <div
                v-for="toast in toasts"
                :key="toast.id"
                class="pointer-events-auto flex items-center gap-3 px-4 py-3 rounded-xl shadow-[0_8px_30px_rgb(0,0,0,0.12)] border backdrop-blur-md min-w-[300px] max-w-sm w-full transition-all"
                :class="getToastClasses(toast.type)"
            >
                <!-- Icon Section (Using Inline SVG to avoid dependencies) -->
                <div class="shrink-0">
                    <!-- Success Icon -->
                    <svg
                        v-if="toast.type === 'success'"
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                        <polyline points="22 4 12 14.01 9 11.01" />
                    </svg>

                    <!-- Error Icon -->
                    <svg
                        v-else-if="toast.type === 'error'"
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <circle cx="12" cy="12" r="10" />
                        <line x1="15" y1="9" x2="9" y2="15" />
                        <line x1="9" y1="9" x2="15" y2="15" />
                    </svg>

                    <!-- Warning Icon -->
                    <svg
                        v-else-if="toast.type === 'warning'"
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <path
                            d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"
                        />
                        <line x1="12" y1="9" x2="12" y2="13" />
                        <line x1="12" y1="17" x2="12.01" y2="17" />
                    </svg>

                    <!-- Info Icon (Default) -->
                    <svg
                        v-else
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <circle cx="12" cy="12" r="10" />
                        <line x1="12" y1="16" x2="12" y2="12" />
                        <line x1="12" y1="8" x2="12.01" y2="8" />
                    </svg>
                </div>

                <!-- Message Content -->
                <div class="flex-1 text-sm font-medium">
                    {{ toast.message }}
                </div>

                <!-- Close Button -->
                <button
                    @click="remove(toast.id)"
                    class="opacity-50 hover:opacity-100 transition p-1"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="16"
                        height="16"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <line x1="18" y1="6" x2="6" y2="18" />
                        <line x1="6" y1="6" x2="18" y2="18" />
                    </svg>
                </button>
            </div>
        </TransitionGroup>
    </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

// --- State Management ---

// Define supported toast types
type ToastType = "success" | "error" | "warning" | "info";

// Interface for a toast object
interface Toast {
    id: number;
    message: string;
    type: ToastType;
}

// Reactive list to store active toast messages
const toasts = ref<Toast[]>([]);
// Counter to ensure unique IDs for each toast
let idCounter = 0;

/**
 * Adds a new toast message to the list.
 * @param message - The text content to display.
 * @param type - The type of toast ('success', 'error', 'warning', 'info').
 * @param duration - Time in milliseconds before auto-dismissal (default: 3000ms).
 */
const add = (
    message: string,
    type: ToastType = "info",
    duration: number = 3000,
) => {
    const id = idCounter++;
    const newToast: Toast = { id, message, type };

    toasts.value.push(newToast);

    // Auto-remove the toast after the specified duration
    if (duration > 0) {
        setTimeout(() => {
            remove(id);
        }, duration);
    }
};

/**
 * Removes a toast from the list by its ID.
 * @param id - The unique ID of the toast to remove.
 */
const remove = (id: number) => {
    const index = toasts.value.findIndex((t) => t.id === id);
    if (index !== -1) {
        toasts.value.splice(index, 1);
    }
};

/**
 * Returns the specific Tailwind CSS classes based on the toast type.
 * @param type - The type of the toast.
 * @returns The CSS class string.
 */
const getToastClasses = (type: string): string => {
    switch (type) {
        case "success":
            return "bg-white/95 text-green-700 border-green-100";
        case "error":
            return "bg-white/95 text-red-600 border-red-100";
        case "warning":
            return "bg-white/95 text-amber-600 border-amber-100";
        case "info":
            return "bg-white/95 text-blue-600 border-blue-100";
        default:
            return "bg-white/95 text-gray-700 border-gray-100";
    }
};

// --- Global Event Listener ---

// Handler for the custom 'show-toast' event
const handleGlobalToastEvent = (event: Event) => {
    const customEvent = event as CustomEvent;
    // Ensure detail exists before accessing
    if (customEvent.detail) {
        const { message, type, duration } = customEvent.detail;
        add(message, type, duration);
    }
};

// Register the event listener when the component mounts
onMounted(() => {
    window.addEventListener("show-toast", handleGlobalToastEvent);
});

// Clean up the event listener when the component unmounts
onUnmounted(() => {
    window.removeEventListener("show-toast", handleGlobalToastEvent);
});
</script>

<style scoped>
/* Vue Transition Styles
  ---------------------
  These classes handle the entry and exit animations for the toast list.
*/

/* Applied during the entire active phase of entering and leaving */
.toast-move,
.toast-enter-active,
.toast-leave-active {
    transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

/* Starting state for enter / Ending state for leave */
.toast-enter-from,
.toast-leave-to {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
}

/* Ensures that elements leaving the DOM are taken out of the layout flow immediately,
  allowing the remaining items to slide up smoothly.
*/
.toast-leave-active {
    position: absolute;
    /* 'w-full' from utility classes might need explicit help here depending on parent */
    width: 100%;
}
</style>
