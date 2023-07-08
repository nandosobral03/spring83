import { API_URL } from "$env/static/private"
import { boardCountStore } from "$lib/stores/board_count.store"

export const load = async () => {
    const board_count: number = await (await fetch(`${API_URL}/boards/count`).then(res => res.json()))
    boardCountStore.set(board_count)
    return {
        board_count
    }
}