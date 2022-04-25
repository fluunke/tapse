import { writable } from 'svelte/store';

export var normal_view = writable(true);

export var username = writable("Anonymous");
export var login_modal = writable(false);

export async function get_session() {
    let res = await fetch(`/api/session`);

    let status = await res.status;

    if (status == 401) {
        login_modal.set(true)
    }

    let data = await res.json();

    username.set(data.username);
}

get_session();

export async function set_session(new_name: string, password: string) {
    let res = fetch(`/api/session`, {
        body: JSON.stringify({
            username: new_name,
            password,
        }),
        headers: {
            'Content-Type': 'application/json'
        },
        method: "POST",
        credentials: "include",

    });

    res.then(async (res) => {
        // Required to reload WS connection with new session    
        window.location.reload();
    })

}


export var current_page = writable(0);

export default {
    set_username: set_session,
    current_page,
}

