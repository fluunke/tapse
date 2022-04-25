<script lang="ts">
    import { rooms } from "../stores/room";
    import { slide } from "svelte/transition";
    import { current_room } from "../stores/room";

    import { MenuIcon, PlusSquareIcon } from "svelte-feather-icons";

    let room_name_input: string = "";
    let room_creation_prompt: boolean = false;

    function create_room() {
        fetch(`/api/rooms`, {
            method: "POST",
            body: JSON.stringify({ room: room_name_input }),
            headers: { "Content-type": "application/json; charset=UTF-8" },
        });

        // reset name input
        room_name_input = "";
        
        room_creation_prompt = false;
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

                    {#if room.unread_messages > 0}
                        <div class="inline-block text-xs text-red-500">
                            {room.unread_messages}
                        </div>
                    {/if} 
                    
                </div>
            </div>
        {/each}

        <hr />
        <div class="pt-1 text-center transition hover:bg-gray-50 rounded-b-md">
            <div
                on:click={() => (room_creation_prompt = !room_creation_prompt)}
                class="px-3 pt-2 pb-3 cursor-pointer"
            >
                <PlusSquareIcon class="inline align-middle" size="16" />
                <div class="inline-block">add room</div>
            </div>
            {#if room_creation_prompt}
                <div transition:slide={{ duration: 100 }} class="w-full">
                    <div class="shadow-md">
                        <input
                            class="h-10 py-1 pb-1 mb-1 text-center rounded-l-md"
                            type="text"
                            bind:value={room_name_input}
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
