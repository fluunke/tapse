import { writable } from 'svelte/store';

let protocol = (window.location.protocol === "https:") ? "wss://" : "ws://";

let socket = new WebSocket(protocol + window.location.host + "/api/ws");
const ws_store = writable('');

// Connection opened
socket.addEventListener('open', function (_ev) {
    console.log("WS Connection established");
});

// Listen for messages
socket.addEventListener('message', function (event) {
    ws_store.set(event.data);
});

export default {
    send(kind: string, value: any) { socket.send(JSON.stringify({ [kind]: value })) },
    websocket_subscribe: ws_store.subscribe,
}