<script lang="ts">
  import type { TFile, TFileInterface } from "src/stores/file";

  import { slide } from "svelte/transition";

  export var file_store: TFile;
  export var file: TFileInterface;
</script>

<li transition:slide={{ duration: 50 }} class="hover:bg-gray-100">
  <a
    href="/api/files/{file.id}/{file.name}"
    class="overflow-x-hidden text-sm hover:underline">{file.name}</a
  >
  <div class="flex flex-row items-center space-x-1">
    <input
      on:click={() => {
        file_store.toggle_selected(file);
      }}
      type="checkbox"
      bind:checked={file.selected}
    />
    <div
      class="float-right text-xs text-gray-300 transition group-hover:text-gray-500"
    >
      {new Date(file.upload_date).toLocaleTimeString()}
    </div>
  </div>
</li>
