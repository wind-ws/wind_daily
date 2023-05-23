import { invoke } from '@tauri-apps/api/tauri'




export function is_path_access() {
    invoke('is_path_access')
        .then((message) => console.log(message))
        .catch((error) => console.error(error))
}