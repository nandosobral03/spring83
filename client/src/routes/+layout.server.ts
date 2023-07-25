import { API_URL } from "$env/static/private"
import { boardCountStore } from "$lib/stores/board_count.store"

export const load = async ({ cookies, request }: { cookies: any, request: any }) => {
    const board_count: number = await (await fetch(`${API_URL}/boards/count`).then(res => res.json()))
    const token = cookies?.get('jwt');

    const orientation = request.headers.get("user-agent")?.toLowerCase().includes("mobile") ? "portrait" : "landscape"

    boardCountStore.set(board_count)
    return {
        board_count,
        token,
        orientation
    } as {
        board_count: number,
        token: string,
        orientation: "portrait" | "landscape"
    }
}