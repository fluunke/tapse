<script context="module" lang="ts">
    export interface TFile {
        id: number;
        name: string;
        room: number;
        upload_date: Date;
    }

    export interface Room {
        id: number;
        name: string;
        creation_date: Date;
        unread_messages?: number;
    }

    export interface Message {
        id: number;
        author: string;
        room: number;
        content: string;
        creation_date: Date;
    }

    export interface User {
        username: string;
    }

    export async function fetch_rooms(): Promise<Array<Room>> {
        const room_response = await fetch(`/api/rooms`);
        return room_response.json().then((x) => x as Array<Room>);
    }

    export function is_room(json: any): boolean {
        return json.hasOwnProperty("new_room");
    }
    export function is_message(json: any): boolean {
        return json.hasOwnProperty("new_message");
    }
    export function is_files(json: any): boolean {
        return json.hasOwnProperty("new_files");
    }

export function is_file_delete(json: any): boolean {
        return json.hasOwnProperty("deleted_file");
    }

    export function is_error(json: any): boolean {
        return json.hasOwnProperty("error");
    }
</script>
