<script lang="ts">
	import Navbar from "./components/Navbar.svelte";
	import Files from "./components/Files.svelte";
	import Chat from "./components/Chat.svelte";
	import ForumView from "./components/ForumView.svelte";
	import Footer from "./components/Footer.svelte";

	import { SvelteToast, toast } from "@zerodevx/svelte-toast";

	import { fade } from "svelte/transition";

	import { login_modal, normal_view, set_session } from "./store.js";
	import { files } from "./stores/file";
	import { current_room, rooms } from "./stores/room";
	import { fetch_messages, messages } from "./stores/message.js";

	import { onMount } from "svelte";
	import { is_error, is_files, is_message, is_room } from "./Models.svelte";
	import RoomBox from "./components/RoomBox.svelte";
	import ws from "./stores/ws.js";

	import type { Message, Room } from "./Models.svelte";

	$: fetch_messages($current_room).then((x) => {
		messages.set(x);
	});

	let new_username: string;
	let code: string;

	onMount(async function () {
		// fetch all data
		const f = await fetch(`/api/files?room=${$current_room}`);
		files.set(await f.json());

		const m = await fetch(`/api/chat?room=${$current_room}`);
		messages.set(await m.json());

		const r = await fetch(`/api/rooms`);
		rooms.set(await r.json());

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
				toast.push(ws_msg.error);
			}

			if (is_message(ws_msg)) {
				let new_msg: Message = ws_msg.new_message;
				if (new_msg.room == $current_room) {
					let msgs = document.getElementById("msgs")!;
					setTimeout(function () {
						msgs.scrollTo(0, msgs.scrollHeight);
					}, 200);

					messages.set([...$messages, new_msg]);
				}
			}

			if (is_files(ws_msg)) {
				let new_files: Array<File> = ws_msg.new_files;

				new_files.forEach((f) => {
					files.set([...$files, f]);
				});
			}

			if (is_room(ws_msg)) {
				let room: Room = ws_msg.new_room;
				rooms.set([...$rooms, room]);
			}
		});
	});
</script>

{#if $login_modal}
	<div
		transition:fade
		class="absolute top-0 left-0 flex items-center justify-center w-full h-full bg-gray-200"
	>
		<div class="flex flex-col p-8 bg-white border rounded-lg shadow-2xl">
			<div class="flex flex-col flex-nowrap">
				<div class="mb-4 font-bold text-lg text-center">
					Tapse login
				</div>
				<input
					class="p-2 border rounded-t-lg"
					placeholder="Username"
					required
					bind:value={new_username}
					type="text"
				/><input
					class="p-2 border"
					placeholder="Access code (if required)"
					bind:value={code}
					type="text"
				/>
			</div>
			<button
				class="p-2 font-bold text-white transition-all bg-blue-400 rounded-b-lg hover:bg-blue-500 hover:shadow-lg"
				on:click={() => {
					set_session(new_username, code);
				}}>Log in</button
			>
		</div>
	</div>
{:else}
	<Navbar />
	<SvelteToast />
	<div class="my-4 text-center">
		<RoomBox />
	</div>
	{#if $normal_view}
		<div
			class="flex flex-row flex-wrap items-baseline justify-center space-x-6 bg-white rounded-xl"
		>
			<Chat />
			<Files />
		</div>
	{:else}
		<ForumView />
	{/if}

	<button on:click={() => normal_view.update((x) => !x)}
		>Toggle view mode</button
	>

	<Footer />
{/if}
