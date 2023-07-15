import { API_URL } from "$env/static/private"
import axios from "axios"
import * as jose from "jose"
import moment from "moment"
export const load = async ({ params, cookies }) => {
    const board = await fetch(`${API_URL}/${params.id}`)
    const body = await board.text()

    const token = cookies?.get("jwt")
    let follows_board = false
    if (token) {
        console.log("TOKEN")
        const decoded: { sub: string, exp: number } = jose.decodeJwt(token) as any;
        if (decoded.exp > moment().unix()) {
            const followed_boards = await axios.get(`${API_URL}/boards/following`, { headers: { Authorization: `${token}` } })
            console.log(followed_boards)
            let body = await followed_boards.data;
            follows_board = body.includes(params.id)
            if (body.includes(params.id)) {
                console.log("FOLLOWED")
            }
            else {
                console.log("NOT FOLLOWED")
            }
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