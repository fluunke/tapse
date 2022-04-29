<script lang="ts">
  import SingleFile from "./files/SingleFile.svelte";
  import type { FileStore, TFile } from "src/stores/file";
  import type { Room } from "src/stores/room";
  import { derived, writable, type Writable } from "svelte/store";

  import Pagination from "./files/Pagination.svelte";
  import UploadForm from "./files/UploadForm.svelte";

  export let file: TFile;
  export let room: Room;

  let file_search = writable("");
  let files_per_page = 10;
  let current_page = writable(0);

  $: plural = (n: number) => (n == 0 || n > 1 ? "s" : "");

  let search_results = derived(
    [file, file_search],
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
        {$search_results.length} file{plural($search_results.length)} of
        {$file.length}:
      {:else}
        {$file.length} file{plural($search_results.length)}
        in this room
      {/if}
    </div>
    <input
      class="w-full h-12 text-center rounded-md shadow-sm"
      bind:value={$file_search}
      on:input={() => {
        $current_page = 0;
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

        {#each $search_results as item, index}
          {#if index < files_per_page * ($current_page + 1) && index + 1 > files_per_page * $current_page}
            <SingleFile file_store={file} file={item} />
          {/if}
        {/each}
      </ul>
    </div>

    <Pagination
      {current_page}
      {files_per_page}
      {pages}
      search_results={$search_results}
    />

    <UploadForm room={$room} file_store={file} />
  </div>
</div>
