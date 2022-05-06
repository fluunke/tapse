<script>
  import { fade } from "svelte/transition";

  export let session;

  let new_username;
  let password;
</script>

<div transition:fade class="flex items-center justify-center w-full h-full">
  <div class="flex flex-col py-16">
    <div class="flex flex-col flex-nowrap ">
      <input
        class="p-2 border rounded-t-lg"
        placeholder="Name"
        required
        bind:value={new_username}
        type="text"
      /><input
        class="p-2 border"
        placeholder="Access code"
        bind:value={password}
        type="text"
      />
    </div>
    <button
      class="p-2 font-bold text-white transition-all bg-blue-400 rounded-b-lg hover:bg-blue-500 hover:shadow-lg"
      on:click={() => {
        session.set_session(new_username, password);
      }}>Log in</button
    >
  </div>
</div>
