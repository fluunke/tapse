<script lang="ts">
    import { onMount } from "svelte";
    import store, { current_room } from "../store.js";
    import { is_files } from "../Models.svelte";
    import type { File } from "../Models.svelte";
    import SingleFile from "./files/SingleFile.svelte";

    import {
        ChevronLeftIcon,
        ChevronsLeftIcon,
        ChevronRightIcon,
        ChevronsRightIcon,
    } from "svelte-feather-icons";

    let files: Array<File> = [];
    let file_search: string = "";
    let files_per_page = 10;
    let current_page = 0;
    // Calculate amount of pages
    $: pages = Math.ceil(show_files.length / files_per_page) - 1;

    // File search
    $: show_files = files.filter((file) =>
        file.name.toLowerCase().includes(file_search.toLowerCase())
    );

    // Fetch files on room change
    $: fetch_files($current_room).then((x) => (files = x));

    function insert_file(f: Array<File>): Array<File> {
        return f.concat(files);
    }

    async function delete_file(file: File) {
        files = files.filter((x) => x !== file);

        await fetch(`/api/files/${file.id}/${file.name}`, {
            method: "DELETE",
        });
    }

    async function fetch_files(room: number): Promise<Array<File>> {
        const file_response = await fetch(`/api/files?room=${room}`);
        return file_response.json();
    }

    onMount(async () => {
        files = await fetch_files($current_room);

        store.websocket_subscribe((socketMessage) => {
            let socketJSON: any = {};

            try {
                socketJSON = JSON.parse(socketMessage);
            } catch {}

            if (is_files(socketJSON)) {
                console.log("should_adda");
                let newfiles: Array<File> = socketJSON.new_files;
                files = insert_file(newfiles);
            }
        });
    });

    async function send_file() {
        let fileForm = document.getElementById("fileUpload");
        let formData = new FormData(fileForm);

        await fetch(`/api/files?room=${$current_room}`, {
            method: "POST",
            body: formData,
        });
    }
</script>

<div class="shadow-xl w-80">
    <div class="p-1 font-bold text-center">
        {#if file_search.length > 0}
            {show_files.length} file{show_files.length == 1 ? "" : "s"} out of
            {files.length}:
        {:else}
            {files.length} file{show_files.length == 1 ? "" : "s"} in this room
        {/if}
    </div>
    <div>
        {#if show_files.length == 0}
            <div class="font-bold text-center text-gray-400 pb-7 text-l">
                No files.
            </div>
        {/if}
        <div
            class="overflow-x-hidden overflow-y-scroll max-h-72 h-72 w-80 divide-y divide-gray-100"
        >
            {#each show_files as file, index}
                {#if index < files_per_page * (current_page + 1) && index >= files_per_page * current_page}
                    <SingleFile {delete_file} {file} />
                {/if}
            {/each}
        </div>

        <div class="flex justify-center">
            <div class="flex flex-row items-center p-1 mb-1 text-center">
                <div>
                    <button
                        class="p-2 transition bg-white border rounded-lg shadow-sm hover:bg-gray-50"
                        on:click={() => {
                            current_page = 0;
                        }}><ChevronsLeftIcon size="16" /></button
                    >

                    <button
                        class="p-2 transition bg-white border rounded-lg shadow-sm hover:bg-gray-50"
                        on:click={() => {
                            if (current_page > 0) {
                                current_page--;
                            }
                        }}><ChevronLeftIcon size="16" /></button
                    >
                </div>
                <div class="px-2">
                    Page {current_page + 1} out of {Math.ceil(
                        show_files.length / files_per_page
                    )}
                </div>

                <div>
                    <button
                        class="p-2 transition bg-white border rounded-lg shadow-sm hover:bg-gray-50"
                        on:click={() => {
                            if (current_page < pages) {
                                current_page++;
                            }
                        }}><ChevronRightIcon size="16" /></button
                    >
                    <button
                        class="p-2 transition bg-white border rounded-lg shadow-sm hover:bg-gray-50"
                        on:click={() => {
                            current_page = pages;
                        }}><ChevronsRightIcon size="16" /></button
                    >
                </div>
            </div>
        </div>

        <input
            class="block p-2 mx-auto mb-2 border border-0 rounded-lg shadow-md"
            type="search"
            bind:value={file_search}
            on:input={() => {
                current_page = 0;
            }}
            placeholder="Search for files"
        />
        <div class="max-w-md pb-2 mx-auto text-center">
            <form
                id="fileUpload"
                name="fileUpload"
                class="pb-3 flex flex-row items-center"
            >
                <input type="file" multiple required name="file" />
                <button
                    on:click|preventDefault={() => {
                        send_file();
                    }}
                    class="px-3 py-2 font-semibold text-white bg-blue-500 rounded-md hover:bg-blue-400"
                    >Upload</button
                >
            </form>
        </div>
    </div>
</div>
