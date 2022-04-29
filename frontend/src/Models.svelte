<script context="module" lang="ts">
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
    return json.hasOwnProperty("delete_file");
  }

  export function is_error(json: any): boolean {
    return json.hasOwnProperty("error");
  }
</script>
