<script lang="ts">
	import Navbar from "./components/Navbar.svelte";
	import Files from "./components/Files.svelte";
	import Chat from "./components/Chat.svelte";
	import Footer from "./components/Footer.svelte";
	import { SvelteToast, toast } from "@zerodevx/svelte-toast";

	import store, { current_room, username } from "./store.js";
	import { onMount } from "svelte";
	import { is_error } from "./Models.svelte";
	import RoomBox from "./components/RoomBox.svelte";

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

<Navbar />
<SvelteToast />
<div class="my-4 text-center">
	Hello, {$username}.
	<RoomBox />
</div>
<div
	class="flex flex-row flex-wrap items-baseline justify-center space-x-6 bg-white rounded-xl"
>
	<Chat />
	<Files />
</div>

<Footer />
