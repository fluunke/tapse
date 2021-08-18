<script lang="typescript">
    import { rooms } from "../stores/room";
    import { slide } from "svelte/transition";
    import { current_room } from "../stores/room";

    import { MenuIcon, PlusSquareIcon } from "svelte-feather-icons";

    let room_name: string = "";
    let add_room_shown: boolean = false;

    function create_room() {
        fetch(`/api/rooms`, {
            method: "POST",
            body: JSON.stringify({ room: room_name }),
            headers: { "Content-type": "application/json; charset=UTF-8" },
        });

        room_name = "";
        add_room_shown = false;
    }
</script>

<div id="room_box" class="max-w-md mx-auto text-center">
    <h2 class="text-lg font-bold">Rooms</h2>
    <div class="mb-1">
        {#each $rooms as room}
            <div
                class="inline-block px-2 mb-2 py-1 transition-all rounded cursor-pointer hover:opacity-60 hover:bg-gray-200"
                on:click={() => {
                    current_room.set(room.id);
                }}
            >
                <div href="/?room={room.name}" class="flex items-center">
                    <MenuIcon class="inline" size="16" />
                    {#if $current_room == room.id}
                        <div class="inline-block text-sm font-bold">
                            {room.name}
                        </div>
                    {:else}
                        <div class="inline-block text-sm">
                            {room.name}
                        </div>
                    {/if}
                </div>
            </div>
        {/each}

        <hr />
        <div class="pt-1 text-center transition hover:bg-gray-50 rounded-b-md">
            <div
                on:click={() => (add_room_shown = !add_room_shown)}
                class="px-3 pt-2 pb-3 cursor-pointer"
            >
                <PlusSquareIcon class="inline align-middle" size="16" />
                <div class="inline-block">add room</div>
            </div>
            {#if add_room_shown}
                <div transition:slide={{ duration: 100 }} class="w-full">
                    <div class="shadow-md">
                        <input
                            class="h-10 py-1 pb-1 mb-1 text-center rounded-l-md"
                            type="text"
                            bind:value={room_name}
                            placeholder="new-room-name"
                        /><button
                            on:click={() => create_room()}
                            class="inline-block h-10 p-2 mb-2 text-white bg-blue-500 rounded-r-md hover:bg-blue-400"
                            >create</button
                        >
                    </div>
                </div>
            {/if}
        </div>
    </div>
</div>
