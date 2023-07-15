import Cookies from "js-cookie";
import { writable } from "svelte/store";
import * as jose from "jose"
import moment from "moment"
export type User = {
    username: string;
    token: string;
}

const createUserStore = () => {
    const { subscribe, set } = writable<User | null>(null);
    return {
        subscribe,
        set: (token: string) => {
            let decoded = jose.decodeJwt(token) as { sub: string, exp: number }
            if (decoded.exp < moment().unix()) {
                Cookies.remove("jwt")
                set(null)
                return
            } else {
                Cookies.set("jwt", token)
                set({ username: decoded.sub, token })
            }
        },
        logout: () => {
            Cookies.remove("jwt")
            set(null)
        }
    }
}



export const userStore = createUserStore();

