import { API_URL } from "$env/static/private"

export const load = async () => {
    let limit = 20
    const recent_boards = await (await fetch(`${API_URL}/boards?offset=0&limit=${limit}`)).json()
    return {
        recent_boards: recent_boards.sort((a: any, b: any) => Math.random() - 0.5),
        offset: limit,
    }

}