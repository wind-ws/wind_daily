import { invoke } from '@tauri-apps/api/tauri'




export function rust_init() {
    invoke('rust_init')
        .then((message) => console.log(message))
        .catch((error) => console.error(error))
}