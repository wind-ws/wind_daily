//! # Module Introduction
//! 
//! `Author          ` : wind-ws 1424343223@qq.com  
//! 
//! `Date            ` : 2023-05-16 20:08:45
//! 
//! `LastAuthor      ` : wind-ws 1424343223@qq.com  
//! 
//! `LastEditTime    ` : 2023-05-17 14:58:45
//! 
//! `FilePath        ` : /src-tauri/src/other/macros.rs
//! 
//! ## Description  : 
//! 一些简简单单的宏,就像我一样朴实无华~



/// 
/// ## macro description : 
/// 简化tauri_install_everything的定义
/// 存在污染性(ta变量),但是无所谓,因为就只有这一个函数,不会污染外界,哈哈哈哈!😆
#[macro_export]
macro_rules! tauri_install_everything {
    (|$($path:ident)|* ) => {
        pub(super) fn tauri_install_everything_(
            mut ta: tauri::Builder<tauri::Wry>)
            ->tauri::Builder<tauri::Wry> {
            $(ta = $path::tauri_install_everything_(ta);)*
            ta
        }
    };
    (|no pub(super) | $($path:ident)|* ) => {
        fn tauri_install_everything_(
            mut ta: tauri::Builder<tauri::Wry>)
            ->tauri::Builder<tauri::Wry> {
            $(ta = $path::tauri_install_everything_(ta);)*
            ta
        }
    };
    (let $ident:ident;$($expr:expr);*) => {
        pub(super) fn tauri_install_everything_(
            mut $ident: tauri::Builder<tauri::Wry>)
            ->tauri::Builder<tauri::Wry> {
            $($ident=$expr;)*
            $ident
        }
    };
    (let $ident:ident;|$($path:ident) | * $($expr:expr);* ) => {
        pub(super) fn tauri_install_everything_(
            mut $ident: tauri::Builder<tauri::Wry>)
            ->tauri::Builder<tauri::Wry> {
            $($ident = $path::tauri_install_everything_($ident);)*
            $($ident=$expr;)*
            $ident
        }
    };
}

