<script lang="ts">
  import Files from "./components/Files.svelte";
  import Chat from "./components/Chat.svelte";
  import Footer from "./components/Footer.svelte";

  import { SvelteToast, toast } from "@zerodevx/svelte-toast";

  import { fade } from "svelte/transition";

  import { onMount } from "svelte";
  import RoomBox from "./components/RoomBox.svelte";
  import ws from "./stores/ws";
  import { Session } from "./stores/session";
  import { Room, type RoomInterface } from "./stores/room";
  import { TFile, type TFileInterface } from "./stores/file";
  import { Message } from "./stores/message";
  import { writable, type Writable } from "svelte/store";

  let session = new Session();
  let room = new Room();
  let file = new TFile($room.current_room);
  let message = new Message($room.current_room);

  let move_file_prompt: Writable<TFileInterface[]> = writable([]);

  // Fetch new data when changing room
  $: changed_room = $room.current_room;
  $: {
    message.fetch_messages(changed_room).then((_) => {});
    file.fetch_files(changed_room).then((_) => {});
  }

  let new_username: string;
  let password: string;

  // Mount websocket connection
  //TODO: Move this to a separate component
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

      let event_kind = Object.keys(ws_msg)[0];

      switch (event_kind) {
        case "error":
          toast.push(ws_msg.error.message);
          break;

        case "incoming_message":
          let new_msg = ws_msg.incoming_message;
          if (new_msg.room == $room.current_room) {
            message.add_message(new_msg);
          } else {
            room.increment_notifications(new_msg.room);
          }
          break;

        case "new_files":
          let new_files: Array<TFile> = ws_msg.new_files;
          if (new_files[0].room == $room.current_room) {
            file.add_files(new_files);
          } else {
            room.increment_notifications(new_files[0].room);
          }
          break;

        case "files_moved":
          let f = ws_msg.files_moved;
          file.remove_files(f.move_files.map((f) => f.id));

          if (f.new_room == $room.current_room) {
            file.add_files(f.move_files);
          } else {
            room.increment_notifications(f.new_room);
          }
          break;

        case "files_deleted":
          file.remove_files([ws_msg.files_deleted]);
          break;

        case "room_created":
          let new_room: RoomInterface = ws_msg.room_created;
          room.add_room(new_room);
          break;

        default:
          console.log("Unhandled event:" + ws_msg.kind);
          break;
      }
    });
  });
</script>

<SvelteToast />

<div
  class="w-full h-full py-2 my-8 bg-white shadow-lg md:h-auto lg:w-2/3 rounded-xl md:w-2/3 md:py-4 xl:w-2/4"
>
  <RoomBox {move_file_prompt} {file} room_store={room} />
  <div
    class="flex flex-col px-4 space-x-2 space-y-8 h-2/3 md:h-full md:flex-row lg:space-y-0"
  >
    <Chat message_store={message} {room} />
    <Files {move_file_prompt} {file} {room} />
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
