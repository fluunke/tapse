import { writable, get, type Subscriber } from 'svelte/store';
import type { Writable } from 'svelte/store';
import ws from './ws';

export interface TFileInterface {
    id: string;
    name: string;
    room: string;
    upload_date: Date;
    selected?: boolean;
}

type FileStore = {
    subscribe: Writable<TFileInterface[]>["subscribe"],
    set: Writable<TFileInterface[]>["set"],
    update: Writable<TFileInterface[]>["update"],
}

export class TFile implements TFileInterface {
    id: string;
    name: string;
    room: string;
    upload_date: Date;
    selected?: boolean;

    store: FileStore = writable([]);
    constructor(room: string) {
        this.fetch_files(room);
    }

    async fetch_files(room: string) {
        const f = await fetch(`/api/files?room=${room}`);
        let json = await f.json();

        this.store.set(json);
    }

    async send_file(room: string, form: HTMLFormElement) {
        let formData = new FormData(form);

        await fetch(`/api/files?room=${room}`, {
            method: "POST",
            body: formData,
        });
    }

    add_files(files: TFileInterface[]) {
        this.store.update(store => ([...files, ...store]));
    }

    async delete_files(files: TFileInterface[]) {

        let f = files.map(f => { return { id: f.id, name: f.name } });
        ws.send("delete_files", f);

        let ids = files.map(f => f.id);
        this.remove_files(ids);
    }

    toggle_selected(file: TFileInterface) {
        this.store.update(store => {
            let map = store.map(f => {
                if (f.id === file.id) {
                    f.selected = !f.selected;
                }
                return f;
            });
            return map;
        });
    }

    selected_files(): TFileInterface[] {
        return get(this.store).filter(f => f.selected);
    }

    deselect_all() {
        this.store.update(store => {
            let map = store.map(f => {
                f.selected = false;
                return f;
            });
            return map;
        });
    }


    move_files(files: TFileInterface[], room: string) {
        // we only need id and name
        let id_name = files.map(f => {
            return {
                id: f.id,
                name: f.name
            }
        });

        let file_move = {
            move_files: id_name,
            new_room: room,
        };

        ws.send("move_files", file_move);
        this.remove_files(files.map(f => f.id));

    }

    remove_files(file_ids: string[]) {
        file_ids.forEach(id => {
            this.store.update(files => {
                return files.filter(f => f.id !== id)
            });
        });
    }

    subscribe(run: Subscriber<TFileInterface[]>) {
        return this.store.subscribe(run);
    }
}
