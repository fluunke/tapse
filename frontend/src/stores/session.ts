import { writable, type Writable } from "svelte/store";

export class Session {

    login_modal: Writable<boolean>;

    constructor() {
        this.login_modal = writable(false);
        this.get_session();
    }

    async get_session() {
        let res = await fetch("/api/session");

        let status = await res.status;

        if (status == 401) {
            this.login_modal.set(true)
        }

        let data = await res.json();
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

        res.then(async (res) => {
            // Required to reload WS connection with new session    
            window.location.reload();
        })

    }

    subscribe(run) {
        return this.login_modal.subscribe(run);
    }

}