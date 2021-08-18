<script lang="ts">
    import { Trash2Icon } from "svelte-feather-icons";

    import { slide } from "svelte/transition";
    import type { TFile } from "../../Models.svelte";
    export var file: TFile;

    export var delete_file = (file: TFile) => {};
</script>

<div
    transition:slide={{ duration: 200 }}
    class="flex items-center justify-between px-4 py-2"
>
    <a
        class="overflow-x-hidden text-sm hover:underline"
        href="/api/files/{file.id}/{file.name}"
    >
        {file.name}</a
    >
    <div class="flex items-center justify-between space-x-1">
        <div
            on:click={() => {
                delete_file(file);
            }}
            class="p-1 border rounded-sm flex"
        >
            <Trash2Icon size="12" />
        </div>
        <div class="float-right text-xs text-gray-500">
            {new Date(file.upload_date).toLocaleTimeString()}
        </div>
    </div>
</div>
