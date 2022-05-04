<script lang="ts">
  import SingleFile from "./files/SingleFile.svelte";
  import type { TFile, TFileInterface } from "src/stores/file";
  import type { Room } from "src/stores/room";
  import { derived, writable, type Writable } from "svelte/store";
  import { slide } from "svelte/transition";
  import Pagination from "./files/Pagination.svelte";
  import UploadForm from "./files/UploadForm.svelte";

  export let file: TFile;
  export let room: Room;
  export let move_file_prompt: Writable<TFileInterface[]>;

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

  // list of currently selected files
  $: selected_files = $file.filter((file: TFileInterface) => file.selected);

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
      placeholder="Search"
      type="text"
      name="file_search"
      id="file_search"
    />
    <div class="h-full space-y-1 overflow-y-scroll">
      <ul
        class="children:(flex items-center justify-between p-2 transition group)"
      >
        <!-- file selection menu -->
        {#if selected_files.length > 0}
          <div
            class="text-center flex flex-col"
            transition:slide={{ duration: 100 }}
          >
            <div class="text-gray-600 pb-2 text-l">
              {selected_files.length} file{plural(selected_files.length)}
              selected
            </div>
            <div
              class="flex flex-row space-x-2 items-center justify-center children:(p-2 text-white transition-all rounded-md shadow-sm)"
            >
              <button
                class="bg-red-400 hover:bg-red-500"
                on:click={() => {
                  file.delete_files(selected_files);
                }}
              >
                Delete
              </button>

              <button
                class="bg-blue-500 hover:bg-blue-600"
                on:click={() => {
                  if ($move_file_prompt.length > 0) {
                    move_file_prompt.set([]);
                  } else {
                    move_file_prompt.set(selected_files);
                  }
                }}
              >
                {#if $move_file_prompt.length > 0}
                  Cancel
                {:else}
                  Move
                {/if}
              </button>

              <button
                class="text-gray-800"
                on:click={() => {
                  file.deselect_all();
                }}
              >
                <svg
                  class="feather feather-x"
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  ><line x1="18" y1="6" x2="6" y2="18" /><line
                    x1="6"
                    y1="6"
                    x2="18"
                    y2="18"
                  /></svg
                >
              </button>
            </div>
          </div>
        {/if}

        {#each $search_results as item, index}
          {#if index < files_per_page * ($current_page + 1) && index + 1 > files_per_page * $current_page}
            <SingleFile file_store={file} file={item} />
          {/if}
        {:else}
          <li class="font-bold text-center text-gray-400 pb-7 text-l">
            No files.
          </li>
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
