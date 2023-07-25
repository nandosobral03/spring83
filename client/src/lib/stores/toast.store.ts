import { writable } from "svelte/store";

type ToastModel = {
    type: "success" | "error" | "warning" | "info",
    title: string,
    text: string,
    timestamp: number
}

type CreateToast = {
    type: "success" | "error" | "warning" | "info",
    title: string,
    text: string,
    duration?: number
}


const createToastStore = () => {
    let state = writable<ToastModel[]>([]);
    return {
        addToast: (toast: CreateToast) => new Promise((resolve, reject) => {
            let { type, title, text, duration = 2000 } = toast;
            let timestamp = new Date().getTime();
            state.update(x => [...x, { type, title, text, timestamp }]);
            setTimeout(() => {
                state.update(x => x.filter(y => y.timestamp !== timestamp));
            }, duration);
        }),
        subscribe: state.subscribe
    }
}

export const toastStore = createToastStore();
