import { emit } from '@tauri-apps/api/event'
import * as appDir from '@tauri-apps/api/path';


/// 注意,在Android中以下有部分Dir是无法获取的,并且一但尝试获取会导致白屏... (这个bug,真离谱,真不好找,🌿!)
/// 由于本app专注android,所以直接注释掉了,虽然可以加catch()解决,但是没必要

export const appCacheDirPath = await appDir.appCacheDir();
export const appConfigDirPath = await appDir.appConfigDir();
export const appDataDirPath = await appDir.appDataDir();
export const appLocalDataDirPath = await appDir.appLocalDataDir();
export const appLogDirPath = await appDir.appLogDir();
export const audioDirPath = await appDir.audioDir();
export const cacheDirPath = await appDir.cacheDir();
export const configDirPath = await appDir.configDir();
export const dataDirPath = await appDir.dataDir();
// export const desktopPath = await appDir.desktopDir();
export const documentDirPath = await appDir.documentDir();
export const downloadDirPath = await appDir.downloadDir();
// export const executableDirPath = await appDir.executableDir();
// export const fontDirPath = await appDir.fontDir();
// export const homeDirPath = await appDir.homeDir();
export const localDataDirPath = await appDir.localDataDir();
export const pictureDirPath = await appDir.pictureDir();
export const publicDirPath = await appDir.publicDir();
export const resourceDirPath = await appDir.resourceDir();
// export const runtimeDirPath = await appDir.runtimeDir();
// export const templateDirPath = await appDir.templateDir();
export const videoDirPath = await appDir.videoDir();
//


/**
 * 用于将 本地目录结构 发送给Rust
 *
 * 不要问我为什么不直接使用Rust处理, 因为2.0.0-alpha似乎压根就没有这个功能
 */


export function path_send_rust() {
    const app_all_base_path = {
        app_cache_dir_path      : appCacheDirPath,
        app_config_dir_path     : appConfigDirPath,
        app_data_dir_path       : appDataDirPath,
        app_local_data_dir_path : appLocalDataDirPath,
        app_log_dir_path        : appLogDirPath,
        audio_dir_path          : audioDirPath,
        cache_dir_path          : cacheDirPath,
        config_dir_path         : configDirPath,
        data_dir_path           : dataDirPath,
        // desktop_path            : desktopPath,
        document_dir_path       : documentDirPath,
        download_dir_path       : downloadDirPath,
        // executable_dir_path     : executableDirPath,
        // font_dir_path           : fontDirPath,
        // home_dir_path           : homeDirPath,
        local_data_dir_path     : localDataDirPath,
        picture_dir_path        : pictureDirPath,
        public_dir_path         : publicDirPath,
        resource_dir_path       : resourceDirPath,
        // runtime_dir_path        : runtimeDirPath,
        // template_dir_path       : templateDirPath,
        video_dir_path          : videoDirPath,
    }
    emit('event_modify_path', app_all_base_path)
}


