<script lang="ts">
  import type { Room } from "src/stores/room";

  import { PlusIcon } from "svelte-feather-icons";

  export let room_store: Room;

  let room_name_input: string = "";
  let room_creation_prompt: boolean = false;
</script>

<div class="relative pb-2" id="room_box">
  <div class="text-lg font-bold text-center">Rooms</div>

  {#if room_creation_prompt}
    <div
      class="absolute flex flex-col items-center justify-center w-full h-full bg-white"
    >
      <input
        class="w-40 p-1 rounded-t-md"
        type="text"
        placeholder="Room name:"
        bind:value={room_name_input}
        name="room_name"
        id="room_name"
      />
      <button
        on:click={() => room_store.create_room(room_name_input)}
        class="w-40 p-1 text-white bg-blue-500 rounded-b-md">Create</button
      >
    </div>
  {/if}
  <ul class="flex flex-wrap justify-center mb-2 space-x-3">
    {#each $room_store.rooms as room}
      <li
        on:click={() => {
          room_store.set_current_room(room.id);
        }}
        class="flex p-1 transition rounded-md cursor-pointer children:self-center hover:bg-gray-200"
      >
        {#if $room_store.current_room == room.id}
          <div class="text-sm font-bold">
            {room.name}
          </div>
        {:else}
          <div class="text-sm">
            {room.name}
          </div>
        {/if}
      </li>
    {/each}

    <li
      on:click={() => (room_creation_prompt = !room_creation_prompt)}
      class="flex p-1 transition rounded-md cursor-pointer children:self-center hover:bg-gray-200"
    >
      <PlusIcon class="text-gray-700" />
    </li>
  </ul>
</div>
