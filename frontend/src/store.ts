import { writable } from 'svelte/store';
import type Message from './Models.svelte';

const messageStore = writable('');

let socket = new WebSocket("ws://" + window.location.host + "/api/ws");
// const socket = new WebSocket('ws://' + window.location.host + '/ws');

// Connection opened
socket.addEventListener('open', function (event) {
    console.log("WS Connection established");
});

// Listen for messages
socket.addEventListener('message', function (event) {
    messageStore.set(event.data);
});

export var username = writable("Anonymous");
export var login_modal = writable(false);

export async function get_session() {
    let res = await fetch(`/api/session`);

    let status = await res.status;

    if (status == 401) {
        login_modal.set(true)
    }

    let data = await res.json();

    username.set(data.username);
}

get_session();


export async function set_session(new_name: string, password: string) {
    let res = await fetch(`/api/session`, {
        body: JSON.stringify({
            username: new_name,
            password,
        }),
        method: "POST",
        credentials: "include",
    });

    let data = await res.json();
    username.set(data.username);

    // Required to reload WS connection with new session    
    window.location.reload();
}

export var current_room = writable(1);

export var current_page = writable(0);

export function set_current_room(room: number) {
    current_room.set(room)
}


export default {
    username,
    set_username: set_session,
    current_room,
    current_page,
    set_current_room,
    websocket_subscribe: messageStore.subscribe,
    send(value: any) { socket.send(JSON.stringify(value)) },
}

