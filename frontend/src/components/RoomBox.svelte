<script lang="ts">
  import type { TFile, TFileInterface } from "src/stores/file";

  import type { Room } from "src/stores/room";
  import type { Writable } from "svelte/store";
  import { slide, fade } from "svelte/transition";

  export let room_store: Room;
  export let file: TFile;

  let room_name_input: string = "";
  let room_creation_prompt: boolean = false;

  export let move_file_prompt: Writable<TFileInterface[]>;
</script>

<div class="relative pb-2" id="room_box">
  <div class="text-lg font-bold text-center">Rooms</div>

  <ul class="flex flex-wrap justify-center mb-2 space-x-3">
    {#each $room_store.rooms as room}
      <li
        class:border-dotted={$move_file_prompt.length > 0}
        class:border-3={$move_file_prompt.length > 0}
        class:border-gray-800={$move_file_prompt.length > 0}
        transition:slide={{ duration: 100 }}
        on:click={() => {
          if ($move_file_prompt.length > 0) {
            file.move_files($move_file_prompt, room.name);
            move_file_prompt.set([]);
          } else {
            room_store.set_current_room(room.name);
          }
        }}
        class="relative flex p-1 transition rounded-md cursor-pointer children:self-center hover:bg-gray-200"
      >
        <div
          class:font-bold={$room_store.current_room == room.name}
          class:font-semibold={room.unread_notifications > 0}
          class="text-sm font-bold"
        >
          {room.name}
        </div>
        {#if room.unread_notifications > 0}
          <span
            transition:fade={{ duration: 300 }}
            class=" inline-block py-1 px-1.5 leading-none text-center text-[0.5rem] whitespace-nowrap align-baseline font-bold bg-red-600 text-white rounded-full absolute top-[0.6rem] -right-[1rem]"
          >
            {room.unread_notifications}
          </span>
        {/if}
      </li>
    {/each}

    <li
      on:click|self={() => (room_creation_prompt = !room_creation_prompt)}
      class="flex p-1 transition rounded-md cursor-pointer children:self-center hover:bg-gray-200"
    >
      {#if room_creation_prompt}
        <div transition:slide={{ duration: 200 }} class="flex flex-row">
          <input
            bind:value={room_name_input}
            class="p-1 border rounded-l-md"
            type="text"
            name="room_name"
          />
          <button
            on:click={() => {
              room_store.create_room(room_name_input);
              room_creation_prompt = false;
            }}
            class="p-1 font-bold text-white bg-blue-500 hover:bg-blue-600 rounded-r-md"
            >Create</button
          >
        </div>
      {/if}
      <svg
        on:click={() => (room_creation_prompt = !room_creation_prompt)}
        class:rotate-45={room_creation_prompt}
        class="text-gray-700 transition transform"
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        ><line x1="12" y1="5" x2="12" y2="19" /><line
          x1="5"
          y1="12"
          x2="19"
          y2="12"
        /></svg
      >
    </li>
  </ul>
</div>
