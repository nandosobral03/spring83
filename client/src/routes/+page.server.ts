import { API_URL } from "$env/static/private"
import type { Board } from "$lib/models/board.model"

export const load = async () => {
    const recent_boards = await(await fetch(`${API_URL}/boards`).then(res => res.json()))
    
    let test_board:Board = {
        body: `
        <style>
            :host {
                background-color: #000;
            }
        </style>
        
        <div style="color: black; font-family: Arial, Helvetica, sans-serif; font-size: 16px; line-height: 1.5; margin: 0; padding: 0;">
            <div style="margin: 0 auto; max-width: 600px;">
                <div style="padding: 20px 0;">
                    <h1 style="font-size: 32px; font-weight: 700; margin: 0;">Hello, world!</h1>
                </div>
                <div style="padding: 20px 0;">
                    <p style="margin: 0;">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam euismod, nisl eget ultricies aliquam, nunc nisl aliquet nunc, quis aliquam nisl nunc vel nisl. Donec euismod, nisl eget ultricies aliquam, nunc nisl aliquet nunc, quis aliquam nisl nunc vel nisl.</p>
                </div>
            </div>
            <img src="https://picsum.photos/600/400" alt="Placeholder image" style="display: block; margin: 0 auto; max-width: 600px; width: 100%;">
        </div>
        <script>
            console.log("Hello, world!")
            alert("Hello, world!")
        </script>

        `,
        timestamp: "",
        last_modified: "",
        signature: "",
        orientation: "Portrait",
        public_key: ""
    }

    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))
    test_board.orientation = Math.random() > 0.5 ? "Portrait" : "Landscape"
    recent_boards.push(JSON.parse(JSON.stringify(test_board)))

    return {
        recent_boards
    }
}