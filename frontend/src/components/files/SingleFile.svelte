<script lang="ts">
  import type { TFile, TFileInterface } from "src/stores/file";

  import { Trash2Icon } from "svelte-feather-icons";
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
      <Trash2Icon size="12" />
    </div>
    <div
      class="float-right text-xs text-gray-300 transition group-hover:text-gray-500"
    >
      {new Date(file.upload_date).toLocaleTimeString()}
    </div>
  </div>
</li>
