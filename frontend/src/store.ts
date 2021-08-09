import { writable } from 'svelte/store';
import type Message from './Models.svelte';

const messageStore = writable('');

let socket = new WebSocket("ws://" + window.location.host + "/ws");
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

export async function get_username() {
    let res = await fetch(`/api/session`);

    let data = await res.json();

    username.set(data.username);
}

get_username();


export async function set_username(new_name: string) {
    let res = await fetch(`/api/session`, {
        body: JSON.stringify({
            username: new_name,
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
    set_username,
    current_room,
    current_page,
    set_current_room,
    websocket_subscribe: messageStore.subscribe,
    send(value: any) { socket.send(JSON.stringify(value)) },
}

