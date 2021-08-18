<script lang="ts">
    import { current_room } from "../stores/room";
    import type { TFile } from "../Models.svelte";
    import SingleFile from "./files/SingleFile.svelte";

    import { files, searched_files, search } from "../stores/file";

    import {
        ChevronLeftIcon,
        ChevronsLeftIcon,
        ChevronRightIcon,
        ChevronsRightIcon,
    } from "svelte-feather-icons";

    let file_search: string = "";
    let files_per_page = 10;
    let current_page = 0;

    $: {
        search.set(file_search);
    }

    // Calculate amount of pages
    $: pages = Math.ceil($searched_files.length / files_per_page) - 1;

    // Fetch files on room change
    $: fetch_files($current_room).then((f) => files.set(f));

    async function delete_file(f: TFile) {
        let res = await fetch(`/api/files/${f.id}/${f.name}`, {
            method: "DELETE",
        });

        if (res.ok) {
            files.set($files.filter((x) => x !== f));
        }
    }

    async function fetch_files(room: number): Promise<Array<TFile>> {
        const file_response = await fetch(`/api/files?room=${room}`);
        return file_response.json();
    }

    async function send_file() {
        let fileForm = document.getElementById("fileUpload");
        let formData = new FormData(fileForm);

        await fetch(`/api/files?room=${$current_room}`, {
            method: "POST",
            body: formData,
        });
    }
</script>

<div class="shadow-xl w-80, rounded-md">
    <div class="p-1 font-bold text-center">
        {#if file_search.length > 0}
            {$searched_files.length} file{$searched_files.length == 1
                ? ""
                : "s"} out of
            {$files.length}:
        {:else}
            {$files.length} file{$searched_files.length == 1 ? "" : "s"} in this
            room
        {/if}
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

    <div>
        {#if $searched_files.length == 0}
            <div class="font-bold text-center text-gray-400 pb-7 text-l">
                No files.
            </div>
        {/if}
        <div
            class="overflow-x-hidden overflow-y-scroll divide-y divide-gray-100 max-h-72 h-72 w-80"
        >
            {#each $searched_files as file, index}
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
                        $searched_files.length / files_per_page
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

        <div class="p-2 text-center">
            <form
                id="fileUpload"
                name="fileUpload"
                class="flex flex-col items-center pb-3"
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
