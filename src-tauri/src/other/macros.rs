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


/// 快捷定义 静态的Rjson变量
#[macro_export]
macro_rules! static_rjson {
    (
        $(#[$meta:meta])*//meta主要是处理 #[doc = "..."]
        pub type $type_name:ident = RJson<$ty:ty>;
        static mut $static_name:ident;
    ) => {
        $(#[$meta])*
        pub type $type_name = RJson<$ty>;
        static mut $static_name:Option<RwLock<$type_name>> =None;
        impl $type_name {
            fn init() {
                unsafe {
                    if let None = $static_name {
                        $static_name = Some(RwLock::new($type_name::updata()));
                    }
                }
            }
            pub fn get_mut_lock()->&'static mut RwLock<$type_name>{
                Self::init();
                unsafe {
                    $static_name.as_mut().unwrap()
                }
            }
            pub fn get_lock()->&'static RwLock<$type_name>{
                Self::init();
                unsafe { 
                    $static_name.as_ref().unwrap() 
                }
            }
        }
    };
}


/// 孵化中的宏 ,禁止使用
mod hatching {
    /// 
    /// ## macro description : 
    /// 利用宏 实现 多重定义
    /// 事实上,它并不完美,甚至可以说非常差, 没办法,我玩宏的极限就在这里了,尝试了很久,只能止步于此了
    /// todo(1) : 实现 [枚举,元组,数组] //好麻烦懒得写, 有一种简单的写法,就是用tt撕咬机,直接整,但是需要用分隔符{},这会导致这样 a:{int},很丑,对吧
    /// todo(0) : 实现 all#[...] 模式 来直接让孩子实现 #[...] 而非一个一个定义  //不会写... 没法传递...
    /// todo(1) : 实现 HashMap<struct{...},struct{...}> ,类似这样的全部
    // #[macro_export]
    macro_rules! multiple_definitions {
        //正常的struct匹配
        (
            $( #[$meta:meta] )*
            $vis:vis struct $name:ident {
                $(
                    $( #[$field_meta:meta] )*
                    $field_vis:vis $field_name:ident : $field_ty:ty
                ),*
                $(,)? 
            }
        ) => {
            $( #[$meta] )*
            $vis struct $name {
                $(
                    $( #[$field_meta] )*
                    $field_vis $field_name : $field_ty
                ),*
            }
        };
        //多重定义struct匹配
        (
            $( #[$meta:meta] )*
            $vis:vis struct $name:ident {
                $(
                    $( #[$field_meta:meta] )*
                    $field_vis:vis $field_name:ident 
                        $(
                            $( #[$child_meta:meta] )*
                            $child_vis:vis struct $child_name:ident {
                                $($tt:tt)*
                            }
                        )?
                        $(:$field_ty:ty )?
                ),*
                $(,)? 
            }
        ) => {
            $( #[$meta] )*
            $vis struct $name {
                $(
                    $( #[$field_meta] )*
                    $field_vis $field_name : $($child_name)? $($field_ty)?
                ),*
            }
            $(
                $(
                    multiple_definitions!(
                        $( #[$child_meta] )*
                        $child_vis struct $child_name{
                            $($tt)*
                        }
                    );
                )? 
            )*
        };
    }


    multiple_definitions!(
        #[allow(dead_code)]
        #[derive(Clone)]
        struct A {
            a1: i32,
            // #[error("aaa")]
            a2  #[allow(dead_code)]
                #[derive(Clone)]
                pub struct B{
                b1:i128,
                b2  #[derive(Clone)]
                    struct C{
                        c:i128
                    },
                b3:i32
            },
            a3: B,
            
        }
    );

}





