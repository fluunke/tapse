import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import ws from './ws';

export type MessageStore = {
    subscribe: Writable<MessageInterface[]>["subscribe"],
    set: Writable<MessageInterface[]>["set"],
    update: Writable<MessageInterface[]>["update"],
}

export interface MessageInterface {
    id: number;
    author: string;
    room: number;
    content: string;
    creation_date: Date;
}

export class Message implements MessageInterface {
    id: number;
    author: string;
    room: number;
    content: string;
    creation_date: Date;

    store: MessageStore;

    constructor(room: number) {
        this.store = writable([]);
        this.fetch_messages(room);
    }

    async fetch_messages(room: number) {
        const msgs = await fetch(`/api/chat?room=${room}`).then(res => res.json());
        this.store.set(msgs);
    }

    async send_message(input: string, room: number) {
        let message = {
            room: room,
            content: input,
        };

        ws.send(message);
    }

    add_message(message: MessageInterface) {
        this.store.update(existingValue => ([...existingValue, message]));

        // scroll to bottom
        let msgs = document.getElementById("msgs")!;
        setTimeout(function () {
            msgs.scrollTo(0, msgs.scrollHeight);
        }, 200);

    }

    subscribe(run) {
        return this.store.subscribe(run);
    }
}