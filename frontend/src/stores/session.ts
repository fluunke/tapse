import { writable } from "svelte/store";

export class Session {

    store = writable({
        login_modal: false,
        username: "",
        id: "",
    });


    constructor() {
        this.get_session();
    }

    async get_session() {
        let res = await fetch("/api/session");
        let status = await res.status;

        if (status == 401) {
            this.store.update(store => ({ ...store, login_modal: true }));
        } else {
            let name = await res.text();
            this.store.update(store => ({ ...store, username: name }));
        }
    };

    async set_session(new_name: string, password: string) {
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

        res.then(async () => {
            // Required to reload WS connection with new session    
            window.location.reload();
        })

    }

    async update_username(new_name: string) {
        let res = await fetch(`/api/session`, {
            body: JSON.stringify({
                username: new_name,
            }),
            headers: {
                'Content-Type': 'application/json'
            },
            method: "PUT",
            credentials: "include",
        });

        let status = await res.status;

        if (status == 200) {
            // I'd really love to skip this one day
            //TODO: Does svelte-websocket-store allow reconnecting?
            window.location.reload();
            // this.store.update(store => ({ ...store, username: new_name }));
            // toast.push("Username updated");
        }

    }

    subscribe(run) {
        return this.store.subscribe(run);
    }

}