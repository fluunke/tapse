<script lang="ts">
  import type { TFile, TFileInterface } from "src/stores/file";

  import { slide } from "svelte/transition";

  export var file_store: TFile;
  export var file: TFileInterface;
</script>

<li transition:slide={{ duration: 100 }} class="hover:bg-gray-100">
  <a
    href="/api/files/{file.id}/{file.name}"
    class="overflow-x-hidden text-sm hover:underline">{file.name}</a
  >
  <div class="flex flex-row items-center">
    <div
      on:click={() => {
        file_store.delete_file(file);
      }}
      class="flex p-1 border rounded-sm"
    >
      <svg
        class="h-4 w-4"
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        ><polyline points="3 6 5 6 21 6" /><path
          d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
        /></svg
      >
    </div>
    <div
      class="float-right text-xs text-gray-300 transition group-hover:text-gray-500"
    >
      {new Date(file.upload_date).toLocaleTimeString()}
    </div>
  </div>
</li>
