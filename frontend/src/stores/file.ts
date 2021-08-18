import { writable, Writable, derived, get } from 'svelte/store';
import type TFile from "../Models.svelte";

type FileStore = {
    subscribe: Writable<TFile[]>["subscribe"],
    set: Writable<TFile[]>["set"]
}

export const files: FileStore = writable([]);
export let search: Writable<string> = writable("");

export const searched_files = derived(
    [search, files],
    // oh no
    ($x, set) => {
        set(
            get(files).filter((f: TFile) =>
                f.name.toLowerCase().includes($x[0].toLowerCase())
            ));
    }
)
