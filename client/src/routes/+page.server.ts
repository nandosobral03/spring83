import { API_URL } from "$env/static/private"

export const load = async () => {
    const recent_boards = await (await fetch(`${API_URL}/boards`).then(res => res.json()))
    console.log(recent_boards)
    return {
        // recent_boards: [...recent_boards, ...recent_boards, ...recent_boards, ...recent_boards, ...recent_boards, ...recent_boards, ...recent_boards, ...recent_boards].sort(
        //     () => Math.random() - 0.5
        // )
        recent_boards: recent_boards
    }

}