<script lang="ts">
  import Login from "./components/Login.svelte";
  import RoomBox from "./components/RoomBox.svelte";
  import Files from "./components/Files.svelte";
  import Chat from "./components/Chat.svelte";
  import Footer from "./components/Footer.svelte";
  import Settings from "./components/settings/Settings.svelte";
  import SettingsButton from "./components/settings/SettingsButton.svelte";

  import { SvelteToast, toast } from "@zerodevx/svelte-toast";

  import { onMount } from "svelte";
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
  let settings_menu_open = writable(false);

  // Fetch new data when changing room
  $: changed_room = $room.current_room;
  $: {
    message.fetch_messages(changed_room).then((_) => {});
    file.fetch_files(changed_room).then((_) => {});
  }

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
  class="w-full h-full relative p-4 bg-white shadow-lg md:h-auto lg:w-2/3 rounded-xl md:w-2/3 xl:w-2/4"
>
  {#if $session.login_modal}
    <Login {session} />
  {:else}
    <RoomBox {move_file_prompt} {file} room_store={room} />

    <SettingsButton {settings_menu_open} />
    <div class="relative flex flex-col mt-4 md:flex-row">
      {#if $settings_menu_open}
        <Settings {session} />
      {/if}
      <Chat message_store={message} {room} />
      <Files {move_file_prompt} {file} {room} />
    </div>
  {/if}
</div>

<Footer />
