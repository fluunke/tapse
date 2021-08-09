<script lang="ts">
    let new_username: string;

    import { set_username } from "../store.js";
</script>

<nav class="w-full px-6 py-4 text-white bg-gray-900 rounded-b-sm shadow">
    <div>
        <a href="/" class="text-2xl text-grey-darkest hover:text-blue-dark"
            >Tapse</a
        >
        <div class="float-right">
            <input
                class="p-2 text-black rounded-l-lg"
                placeholder="Username"
                bind:value={new_username}
                type="text"
            /><button
                class="p-2 font-bold text-white bg-blue-400 rounded-r-lg hover:bg-blue-500"
                on:click={() => {
                    set_username(new_username);
                }}>Set</button
            >
        </div>
    </div>
</nav>
