<script lang="ts">
	import Navbar from "./components/Navbar.svelte";
	import Files from "./components/Files.svelte";
	import Chat from "./components/Chat.svelte";
	import Footer from "./components/Footer.svelte";
	import { SvelteToast, toast } from "@zerodevx/svelte-toast";

	import { fade } from "svelte/transition";

	import store, { login_modal, set_session } from "./store.js";
	import { onMount } from "svelte";
	import { is_error } from "./Models.svelte";
	import RoomBox from "./components/RoomBox.svelte";

	let new_username: string;
	let password: string;

	onMount(() => {
		store.websocket_subscribe((socketMessage) => {
			let socketJSON: any = {};
			try {
				socketJSON = JSON.parse(socketMessage);
			} catch {}

			if (is_error(socketJSON)) {
				toast.push(socketJSON.error);
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
			<div class="flex flex-nowrap">
				<input
					class="p-2 border rounded-tl-lg"
					placeholder="Username"
					required
					bind:value={new_username}
					type="text"
				/><input
					class="p-2 border rounded-tr-lg"
					placeholder="Password (if required)"
					bind:value={password}
					type="text"
				/>
			</div>
			<button
				class="p-2 font-bold text-white transition-all bg-blue-400 rounded-b-lg hover:bg-blue-500 hover:shadow-lg"
				on:click={() => {
					set_session(new_username, password);
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
	<div
		class="flex flex-row flex-wrap items-baseline justify-center space-x-6 bg-white rounded-xl"
	>
		<Chat />
		<Files />
	</div>

	<Footer />
{/if}
