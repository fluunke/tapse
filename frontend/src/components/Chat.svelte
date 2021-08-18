<script lang="typescript">
    import { slide } from "svelte/transition";
    import { SendIcon } from "svelte-feather-icons";
    import { messages } from "../stores/message";
    import { current_room } from "../stores/room";
    import ws from "../stores/ws";

    let message_input: string = "";

    async function send_message(input: string) {
        let message = {
            room: $current_room,
            content: input,
        };
        ws.send(message);

        message_input = "";
    }

    function enterHandler(key: any, type: string) {
        if (key.keyCode == 13) {
            switch (type) {
                case "message":
                    send_message(message_input);
                    break;
            }
        }
    }
</script>

<div class="shadow-xl w-80 rounded-md">
    <div class="p-1 font-bold text-center">Chat</div>

    <div
        id="msgs"
        class="overflow-y-scroll divide-y divide-gray-100 max-h-72 h-72"
    >
        {#if $messages.length == 0}
            <div class="font-bold text-center text-gray-400 pb-7 text-l">
                No messages yet!
            </div>
        {/if}

        {#each $messages as message}
            <div
                transition:slide={{ duration: 200 }}
                class="block pt-3 pb-3 pl-4 pr-4 text-sm break-words"
            >
                <b>{message.author}:</b>
                {message.content}
                <div class="float-right text-xs text-gray-500">
                    {new Date(message.creation_date).toLocaleTimeString()}
                </div>
            </div>
        {/each}
    </div>
    <div id="submit" class="flex mx-auto rounded-md">
        <input
            type="text"
            class="flex-1 h-10 px-4 border border-gray-200 rounded-bl-lg"
            name="msg"
            bind:value={message_input}
            placeholder="Write a message..."
            id="msg_input"
            on:keydown={(e) => enterHandler(e, "message")}
        />
        <div
            id="submit_button"
            on:click={() => send_message(message_input)}
            class="flex w-10 h-10 align-bottom bg-blue-500 rounded-br-lg cursor-pointer hover:bg-blue-600"
        >
            <i
                class="flex items-center content-center justify-center w-10 h-10 text-white"
                ><SendIcon size="16" /></i
            >
        </div>
    </div>
</div>
