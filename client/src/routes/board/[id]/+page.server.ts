import { API_URL } from "$env/static/private"

export const load = async ({ params }) => {
    const board = await fetch(`${API_URL}/${params.id}`)
    const body = await board.text()
    return {
        body: body,
        orientation: board.headers.get("orientation"),
        lastModified: board.headers.get("last-modified"),
        signature: board.headers.get("spring-signature"),
        size: board.headers.get("content-length")
    }
}