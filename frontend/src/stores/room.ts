import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type Room from "../Model.svelte";

export const current_room = writable(1);

export type RoomStore = {
    subscribe: Writable<Room[]>["subscribe"],
    set: Writable<Room[]>["set"]
}

export const rooms: RoomStore = writable([]);
