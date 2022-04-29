<script lang="ts">
  import type { TFile } from "src/stores/file";

  export let file_store: TFile;
  export let room;
</script>

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
        room.current_room,
        document.getElementById("fileUpload")
      );
    }}>Upload</button
  >
</form>
