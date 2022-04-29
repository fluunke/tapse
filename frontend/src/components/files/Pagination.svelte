<script lang="ts">
  export let current_page;
  export let search_results;
  export let files_per_page;
  export let pages;
</script>

<div class="flex flex-row items-center justify-center p-1 mb-1 text-center">
  <div>
    <button
      class="p-2 transition bg-white border rounded-lg shadow-sm hover:bg-gray-50"
      on:click={() => {
        $current_page = 0;
      }}
      ><svg
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
        ><polyline points="11 17 6 12 11 7" /><polyline
          points="18 17 13 12 18 7"
        /></svg
      ></button
    >

    <button
      class="p-2 transition bg-white border rounded-lg shadow-sm hover:bg-gray-50"
      on:click={() => {
        if ($current_page > 0) {
          $current_page--;
        }
      }}
      ><svg
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
        ><polyline points="11 17 6 12 11 7" /><polyline
          points="18 17 13 12 18 7"
        /></svg
      ></button
    >
  </div>
  <div class="px-2">
    Page {$current_page + 1} of {Math.ceil(
      search_results.length / files_per_page
    )}
  </div>

  <div>
    <button
      class="p-2 transition bg-white border rounded-lg shadow-sm hover:bg-gray-50"
      on:click={() => {
        if ($current_page < pages) {
          $current_page++;
        }
      }}
      ><svg
        class="h-4 w-4"
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"><polyline points="9 18 15 12 9 6" /></svg
      ></button
    >
    <button
      class="p-2 transition bg-white border rounded-lg shadow-sm hover:bg-gray-50"
      on:click={() => {
        $current_page = pages;
      }}
      ><svg
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
        ><polyline points="13 17 18 12 13 7" /><polyline
          points="6 17 11 12 6 7"
        /></svg
      ></button
    >
  </div>
</div>
