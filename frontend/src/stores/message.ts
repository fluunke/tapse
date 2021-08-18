import { writable, Writable } from 'svelte/store';
import type Message from './Models.svelte';

export async function fetch_messages(
    room: number
): Promise<Array<Message>> {
    const msgs = await fetch(`/api/chat?room=${room}`);
    return msgs.json();
}

export type MessageStore = {
    subscribe: Writable<Message[]>["subscribe"],
    set: Writable<Message[]>["set"]
}

export const messages: MessageStore = writable([]);
