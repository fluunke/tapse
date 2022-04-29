<script lang="ts">
  import type { Message } from "src/stores/message";
  import type { Room } from "src/stores/room";

  import { slide } from "svelte/transition";

  let message_input: string = "";

  export let message_store: Message;
  export let room: Room;

  let { store } = message_store;

  function enter_handler(key: any, type: string) {
    if (key.keyCode == 13) {
      switch (type) {
        case "message":
          message_store.send_message(message_input, $room.current_room);
          message_input = "";
      }
    }
  }
</script>

<div id="messages" class="flex flex-col space-y-2 h-2/4 md:h-96 basis-1/2">
  <div class="m-2 text-lg font-bold text-center">Chat</div>
  <div id="msgs" class="h-full space-y-1 overflow-y-scroll">
    {#if $store.length == 0}
      <div class="font-bold text-center text-gray-400 pb-7 text-l">
        No messages.
      </div>
    {/if}
    <ul
      class="children:(flex items-center justify-between p-2 transition group)"
    >
      {#each $store as message}
        <li transition:slide={{ duration: 100 }} class="hover:bg-gray-100">
          <div>
            <span class="text-sm font-bold">{message.author}:</span>
            <span class="text-xs break-all">{message.content}</span>
          </div>
          <div
            class="float-right text-xs text-gray-300 transition group-hover:text-gray-500"
          >
            {new Date(message.creation_date).toLocaleTimeString()}
          </div>
        </li>
      {/each}
    </ul>
  </div>
  <div class="grid h-12 grid-cols-8">
    <input
      placeholder="Write a message"
      class="col-start-1 col-end-8 p-2 border rounded-l-lg focus:placeholder:text-gray-200"
      type="text"
      bind:value={message_input}
      on:keydown={(e) => enter_handler(e, "message")}
      name="message_input"
      id="message_input"
    />
    <button
      on:click={() => {
        message_store.send_message(message_input, $room.current_room);
        message_input = "";
      }}
      class="col-start-8 col-end-9 font-bold text-white bg-blue-500 rounded-r-lg hover:bg-blue-600"
      >Send</button
    >
  </div>
</div>
