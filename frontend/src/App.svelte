<script lang="ts">
  import Files from "./components/Files.svelte";
  import Chat from "./components/Chat.svelte";
  import Footer from "./components/Footer.svelte";

  import { SvelteToast, toast } from "@zerodevx/svelte-toast";

  import { fade } from "svelte/transition";

  import { onMount } from "svelte";
  import {
    is_error,
    is_files,
    is_file_delete,
    is_message,
    is_room,
  } from "./Models.svelte";
  import RoomBox from "./components/RoomBox.svelte";
  import ws from "./stores/ws";

  import { Session } from "./stores/session";
  import { Room, type RoomInterface } from "./stores/room";
  import { TFile } from "./stores/file";
  import { Message, type MessageInterface } from "./stores/message";

  let session = new Session();
  let room = new Room();
  let file = new TFile($room.current_room);
  let message = new Message($room.current_room);

  // Fetch new data when changing room
  $: message.fetch_messages($room.current_room).then((_) => {});
  $: file.fetch_files($room.current_room).then((_) => {});

  let new_username: string;
  let password: string;

  // Mount websocket connection
  onMount(async function () {
    ws.websocket_subscribe((socketMessage) => {
      let ws_msg: any = {};
      try {
        ws_msg = JSON.parse(socketMessage);
      } catch {
        // receives empty message on init
        if (socketMessage.length > 1) {
          console.error(socketMessage);
        }
      }

      if (is_error(ws_msg)) {
        toast.push(ws_msg.error.message);
      }

      if (is_message(ws_msg)) {
        let new_msg: MessageInterface = ws_msg.new_message;
        if (new_msg.room == $room.current_room) {
          message.add_message(new_msg);
        } else {
          //TODO: handle unread messages
          // I can't find a neat way to access a single room item from here.
        }
      }

      if (is_files(ws_msg)) {
        let new_files: Array<TFile> = ws_msg.new_files;
        file.add_files(new_files);
      }

      if (is_file_delete(ws_msg)) {
        let deleted_file: string = ws_msg.delete_file;
        file.remove_file(deleted_file);
      }

      if (is_room(ws_msg)) {
        let new_room: RoomInterface = ws_msg.new_room;
        room.add_room(new_room);
      }
    });
  });
</script>

<SvelteToast />

<div
  class="w-full h-full py-2 my-8 bg-white shadow-lg md:h-auto lg:w-2/3 rounded-xl md:w-2/3 md:py-4 xl:w-2/4"
>
  <RoomBox room_store={room} />
  <div
    class="flex flex-col px-4 space-x-2 space-y-8 h-2/3 md:h-full md:flex-row lg:space-y-0"
  >
    <Chat message_store={message} {room} />
    <Files file_store={file} {room} />
  </div>
</div>

{#if $session}
  <div
    transition:fade
    class="absolute top-0 left-0 flex items-center justify-center w-full h-full bg-gray-200"
  >
    <div class="flex flex-col p-8 bg-white border rounded-lg shadow-2xl">
      <div class="flex flex-col flex-nowrap">
        <div class="mb-4 text-lg font-bold text-center">Tapse login</div>
        <input
          class="p-2 border rounded-t-lg"
          placeholder="Username"
          required
          bind:value={new_username}
          type="text"
        /><input
          class="p-2 border"
          placeholder="Access code (if required)"
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
{/if}

<Footer />
