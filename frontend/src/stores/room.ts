import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';

export type RoomStore = {
    subscribe: Writable<{ current_room: string, rooms: RoomInterface[] }>["subscribe"],
    set: Writable<{ current_room: string, rooms: RoomInterface[] }>["set"]
    update: Writable<{ current_room: string, rooms: RoomInterface[] }>["update"]
}

export interface RoomInterface {
    name: string;
    creation_date: Date;
    unread_notifications?: number;
}

export class Room implements RoomInterface {
    name: string;
    creation_date: Date;
    unread_notifications?: number;

    store: RoomStore = writable({
        current_room: "",
        rooms: [],
    });

    constructor() {
        this.update_rooms();
    }

    set_current_room(room: string) {
        localStorage.setItem('last_room', room);

        this.store.update(store => ({ ...store, current_room: room }));
        this.reset_notifications(room);
    }

    increment_notifications(room_name: string) {
        this.store.update(store => {
            let rooms = store.rooms.map(room => {
                if (room.name == room_name) {
                    room.unread_notifications = room.unread_notifications ? room.unread_notifications + 1 : 1;
                }
                return room;
            });
            return { ...store, rooms };
        });
    }

    reset_notifications(room_name: string) {
        this.store.update(store => {
            let rooms = store.rooms.map(room => {
                if (room.name == room_name) {
                    room.unread_notifications = 0;
                }
                return room;
            });
            return { ...store, rooms };
        });
    }

    async update_rooms() {
        const r = await fetch(`/api/rooms`);
        let json: RoomInterface[] = await r.json();

        let last_room;

        if (!localStorage.getItem('last_room')) {
            last_room = json[0].name;
        } else {
            last_room = localStorage.getItem('last_room');
        }

        this.store.update(store => ({ ...store, rooms: json, current_room: last_room }));
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