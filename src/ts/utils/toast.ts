/**
 * Supported toast types
 */
export type ToastType = "success" | "error" | "warning" | "info";

/**
 * Trigger a toast message globally
 * @param message The text content to show
 * @param type The style type
 * @param duration How long to stay (ms)
 */
export const toast = (
    message: string,
    type: ToastType = "info",
    duration: number = 3000,
) => {
    window.dispatchEvent(
        new CustomEvent("show-toast", {
            detail: { message, type, duration },
        }),
    );
};

/**
 * Convenience helpers for different toast states
 */
toast.success = (msg: string, dur?: number) => toast(msg, "success", dur);
toast.error = (msg: string, dur?: number) => toast(msg, "error", dur);
toast.warning = (msg: string, dur?: number) => toast(msg, "warning", dur);
toast.info = (msg: string, dur?: number) => toast(msg, "info", dur);
