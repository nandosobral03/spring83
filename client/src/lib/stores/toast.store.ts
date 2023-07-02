import { writable } from "svelte/store";

type ToastModel = {
    kind: "success" | "error" | "warning" | "info",
    title: string,
    text: string,
    timestamp: number
}


const createToastStore = () => {
    let state = writable<ToastModel[]>([]);
    return {
        addToast: ((kind: "success" | "error" | "warning" | "info", title: string, text: string, duration: number = 2000) => {
            let timestamp = new Date().getTime();
            state.update(x => [...x, { kind, title, text, timestamp }]);
            console.log(state);
            setTimeout(() => {
                state.update(x => x.filter(y => y.timestamp !== timestamp));
            }, duration);
        }),
        subscribe: state.subscribe
    }
}

export const toastStore = createToastStore();

export default toastStore;