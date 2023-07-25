import type { SvelteComponent } from 'svelte'
import { writable } from 'svelte/store'

export type ModalModel = {
    title: string,
    component: any,
    props: any
    size: 'sm' | 'md' | 'lg';
}

export const currentModalStore = writable<ModalModel | null>(null)


const createModalStore = () => {
    const { subscribe, set, update } = writable<ModalModel[]>([])
    let modals: ModalModel[] = []
    return {
        subscribe,
        add: (modal: ModalModel) => {
            update(modals => [...modals, modal])
            modals.push(modal)
            currentModalStore.set(modal)
        },
        pop: () => {
            update(modals => modals.slice(0, -1))
            modals.pop()
            currentModalStore.set(modals[modals.length - 1])
        },
        clear: () => {
            set([])
            modals = []
            currentModalStore.set(null)
        }
    }
}


export const modalStore = createModalStore()
