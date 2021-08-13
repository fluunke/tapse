<script lang="typescript">
    import { is_message } from "../Models.svelte";
    import type { Message } from "../Models.svelte";
    import store, { current_room } from "../store";

    import { slide } from "svelte/transition";
    import { SendIcon } from "svelte-feather-icons";
    import { onMount } from "svelte";

    let message_input: string = "";

    let messages: Array<Message> = [];

    let files: Array<File> = [];
    let file_search: string = "";

    // Re-fetch messages when changing room
    $: fetch_messages($current_room).then((x) => {
        messages = x;
    });

    async function send_message(input: string) {
        let message = {
            room: $current_room,
            content: input,
        };
        store.send(message);
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

    export function insert_message(message: Message): Array<Message> {
        if (message.room == $current_room) {
            return [...messages, message];
        } else {
            return messages;
        }
    }

    export async function fetch_messages(
        room: number
    ): Promise<Array<Message>> {
        const message_response = await fetch(`/api/chat?room=${room}`);

        return message_response.json().then((x) => x as Array<Message>);
    }

    // Subscribe to messages
    onMount(async function () {
        messages = await fetch_messages($current_room);

        store.websocket_subscribe((socketMessage) => {
            let ws_json: any = {};
            try {
                ws_json = JSON.parse(socketMessage);
            } catch {}

            if (is_message(ws_json)) {
                console.log("well its a msg yes");
                let new_msg: Message = ws_json.new_message;
                console.log(new_msg);
                messages = insert_message(new_msg);
            }
        });
    });
</script>

<div class="shadow-xl w-80 rounded-b-md">
    <div class="p-1 font-bold text-center">Chat</div>

    <div
        id="msgs"
        class="overflow-y-scroll divide-y divide-gray-100 max-h-72 h-72"
    >
        {#if messages.length == 0}
            <div class="font-bold text-center text-gray-400 pb-7 text-l">
                No messages yet!
            </div>
        {/if}

        {#each messages as message}
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
