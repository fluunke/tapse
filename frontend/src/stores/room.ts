import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';


export type RoomStore = {
    subscribe: Writable<{ current_room: number, rooms: RoomInterface[] }>["subscribe"],
    set: Writable<{ current_room: number, rooms: RoomInterface[] }>["set"]
    update: Writable<{ current_room: number, rooms: RoomInterface[] }>["update"]
}

export interface RoomInterface {
    id: number;
    name: string;
    creation_date: Date;
    unread_messages?: number;
}

export class Room implements RoomInterface {
    id: number;
    name: string;
    creation_date: Date;
    unread_messages?: number;

    store: RoomStore = writable({
        current_room: 1,
        rooms: [],
    });

    constructor() {
        this.fetch_rooms();
    }

    set_current_room(room: number) {
        this.store.update(store => ({ ...store, current_room: room }));
    }

    async fetch_rooms() {
        const r = await fetch(`http://localhost:8080/api/rooms`);
        let json: RoomInterface[] = await r.json();

        this.store.update(store => ({ ...store, rooms: json }));

    }

    async create_room(name: string) {
        fetch(`/api/rooms`, {
            method: "POST",
            body: JSON.stringify({ room: name }),
            headers: { "Content-type": "application/json; charset=UTF-8" },
        });
    }

    add_room(room: RoomInterface) {
        this.store.update(store => ({ ...store, rooms: [...store.rooms, room] }));
    }

    subscribe(run) {
        return this.store.subscribe(run);
    }
}