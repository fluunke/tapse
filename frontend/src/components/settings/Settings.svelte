<script lang="ts">
  import type { Session } from "src/stores/session";

  import { fade } from "svelte/transition";
  export let session: Session;
  let new_username = $session.username;
</script>

<div
  class="absolute h-full w-full flex flex-col p-8 bg-white"
  transition:fade={{ duration: 100 }}
>
  <h2 class="font-bold text-2xl mb-2">Settings</h2>
  <ul>
    <li>
      <label for="username">Change Username</label>
      <input
        class="p-2 border rounded-l-lg focus:placeholder:text-gray-200"
        bind:value={new_username}
        type="text"
        name="username"
        id="username"
      />
      <button
        class="p-2 bg-blue-500 hover:bg-blue-600 transition rounded-r-md text-white disabled:bg-gray-300 disabled:text-black"
        disabled={new_username == $session.username}
        on:click={() => {
          session.update_username(new_username);
        }}>Update</button
      >
    </li>
    <!-- <li>
      <label for="password">Server password</label>
      <input type="text" name="password" id="password" />
    </li> -->
  </ul>
</div>
