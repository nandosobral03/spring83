import { PUBLIC_API_URL } from "$env/static/public";
import axios from "axios";
import { writable } from "svelte/store";


export let refreshBoardCount = async () => {
    const count = await axios.get(`${PUBLIC_API_URL}/boards/count`);
    boardCountStore.set(count.data);
}
export let boardCountStore = writable(0);