<script lang="ts">
  import SingleFile from "./files/SingleFile.svelte";
  import type { Room } from "src/stores/room";
  import { derived, writable } from "svelte/store";

  import {
    ChevronLeftIcon,
    ChevronsLeftIcon,
    ChevronRightIcon,
    ChevronsRightIcon,
  } from "svelte-feather-icons";

  export let file_store;
  export let room: Room;

  let file_search = writable("");
  let files_per_page = 10;
  let current_page = 0;

  $: plural = (n: number) => (n == 0 || n > 1 ? "s" : "");

  let search_results = derived(
    [file_store, file_search],
    ([$file_store, $file_search]) => {
      let files = $file_store;
      let search = $file_search.toLowerCase();

      if (search.length > 0) {
        files = files.filter((file) => {
          return file.name.toLowerCase().includes(search);
        });
      }
      return files;
    }
  );

  // Calculate amount of pages
  $: pages = Math.ceil($search_results.length / files_per_page) - 1;
</script>

<div class="basis-1/2" id="files">
  <div class="flex flex-col space-y-2 h-2/4 md:h-96">
    <div class="text-lg font-bold text-center">
      {#if $file_search.length > 0}
        {$search_results.length} file{plural($search_results.length)} out of
        {$file_store.length}:
      {:else}
        {$file_store.length} file{plural($search_results.length)}
        in this room
      {/if}
    </div>
    <input
      class="w-full h-12 text-center rounded-md shadow-sm"
      bind:value={$file_search}
      on:input={() => {
        current_page = 0;
        file_search = this.value;
      }}
      placeholder="Search for files"
      type="text"
      name="file_search"
      id="file_search"
    />
    <div class="h-full space-y-1 overflow-y-scroll">
      <ul
        class="children:(flex items-center justify-between p-2 transition group)"
      >
        {#if $search_results.length == 0}
          <li class="font-bold text-center text-gray-400 pb-7 text-l">
            No files.
          </li>
        {/if}

        {#each $search_results as file, index}
          {#if index < files_per_page * (current_page + 1) && index >= files_per_page * current_page}
            <SingleFile {file_store} {file} />
          {/if}
        {/each}
      </ul>
    </div>
    <div class="flex flex-row items-center justify-center p-1 mb-1 text-center">
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
          $search_results.length / files_per_page
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
    <form
      class="flex items-center justify-center h-12"
      id="fileUpload"
      name="fileUpload"
    >
      <input
        class="text-xs file:text-xs"
        type="file"
        name="file_input"
        id="file_input"
        multiple
        required
      />
      <button
        class="h-10 px-4 py-1 font-bold text-white bg-blue-500 rounded-md hover:bg-blue-600"
        on:click|preventDefault={() => {
          file_store.send_file(
            $room.current_room,
            document.getElementById("fileUpload")
          );
        }}>Upload</button
      >
    </form>
  </div>
</div>
