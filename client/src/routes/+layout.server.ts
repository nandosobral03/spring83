import { API_URL } from "$env/static/private"

export const load = async () => {
    const board_count :number = await(await fetch(`${API_URL}/boards/count`).then(res => res.json()))
    return {
        board_count
    }
}