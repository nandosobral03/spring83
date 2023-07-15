import { API_URL } from "$env/static/private"
import { boardCountStore } from "$lib/stores/board_count.store"

export const load = async ({ cookies }) => {
    const board_count: number = await (await fetch(`${API_URL}/boards/count`).then(res => res.json()))
    const token = cookies?.get('jwt');

    boardCountStore.set(board_count)
    return {
        board_count,
        token
    }
}