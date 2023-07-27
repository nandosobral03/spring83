import { API_URL } from "$env/static/private"
import { error } from "@sveltejs/kit"
import axios from "axios"
import * as jose from "jose"
import moment from "moment"
export const load = async ({ params, cookies }) => {
    const board = await fetch(`${API_URL}/${params.id}`)
    if (board.status !== 200) throw error(404, {
        message: 'Not found'
    });
    const body = await board.text()

    const token = cookies?.get("jwt")
    let follows_board = false
    if (token) {
        const decoded: { sub: string, exp: number } = jose.decodeJwt(token) as any;
        if (decoded.exp > moment().unix()) {
            const followed_boards = await axios.get(`${API_URL}/boards/following`, { headers: { Authorization: `${token}` } })
            follows_board = followed_boards.data.includes(params.id)
        }
    }

    return {
        body: body,
        orientation: board.headers.get("orientation"),
        lastModified: board.headers.get("last-modified"),
        signature: board.headers.get("spring-signature"),
        size: board.headers.get("content-length"),
        id: params.id,
        following: follows_board
    }
}