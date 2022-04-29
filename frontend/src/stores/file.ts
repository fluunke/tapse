import { writable, derived, get } from 'svelte/store';
import type { Writable } from 'svelte/store';

export interface TFileInterface {
    id: string;
    name: string;
    room: number;
    upload_date: Date;
}

type FileStore = {
    subscribe: Writable<TFileInterface[]>["subscribe"],
    set: Writable<TFileInterface[]>["set"],
    update: Writable<TFileInterface[]>["update"],
}

export class TFile implements TFileInterface {
    id: string;
    name: string;
    room: number;
    upload_date: Date;


    store: FileStore = writable([]);

    constructor(room: number) {
        this.store = writable([]);
    }

    async fetch_files(room: number) {
        const f = await fetch(`/api/files?room=${room}`);
        let json = await f.json();

        this.store.set(json);
    }

    async send_file(room: number, form: HTMLElement) {
        let formData = new FormData(form);

        await fetch(`/api/files?room=${room}`, {
            method: "POST",
            body: formData,
        });
    }

    add_files(files: TFileInterface[]) {
        this.store.update(store => ([...store, ...files]));
    }


    async delete_file(file: TFileInterface) {

        let res = await fetch(`/api/files/${file.id}/${file.name}`, {
            method: "DELETE",
        });

        if (res.ok) {
            this.remove_file(file.id);
        }
    }

    remove_file(file: string) {
        this.store.update(files => {
            return files.filter(f => f.id !== file)
        });
    }

    subscribe(run) {
        return this.store.subscribe(run);
    }
}
