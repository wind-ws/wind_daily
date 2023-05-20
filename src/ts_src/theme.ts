/// 主题控制
/// 因为各种各样的插件,不同的主题,所以需要统一控制,他们的主题
/// 若您是UI大佬,欢迎来贡献主题~
/// 说实话,关于UI,我不仅啥也不懂 而且没有精力去管理和配置主题样式
/// 甚至是 Light和Dark 我只有精力去管理Dark主题
import { ref ,Ref} from 'vue'



export enum EnumTheme {
    // System,//跟随系统,还是不要它了,好麻烦的~
    Light,
    Dark,
    Cofe,//只有当所有插件的主题都配置好 Cofe 后才可启用Cofe主题
}

type set_theme_f = (theme:EnumTheme)=>void;

/// 若K1在K2中存在 则为U否则为S
type same_key<K1,K2,U,S> = K1 extends K2 ? U : S;
type enum_theme_key = keyof typeof EnumTheme;
type empty=null|undefined|never;
type get_empty<T> = {
    [k in keyof T as T[k] extends empty ?k:never] : T[k]
};
type enum_theme_object<T> = {
    [k in enum_theme_key]?:T
};
/// 这个类型 允许你 通过V来限定EnumTheme中的字段类型 , 若省略定义为S
/// 可以将 不想定义的 字段 在 V 中定义为 null|undefined|never ,即可省略定义
export type enum_theme<V extends enum_theme_object<any> = {},S=any> = {
    readonly [k in enum_theme_key as k extends keyof get_empty<V> ? never : k ]:
        same_key<k,keyof V,V[k],S>
} & {readonly set_theme_f:set_theme_f};

// 这体操玩的...玩的 我糊里糊涂~🥴🥴🥴

export let vant_theme_status=ref("dark");
const vant_theme:enum_theme<{Cofe:never},string> = {
    Light:"light",
    Dark:"dark",
    set_theme_f(theme:EnumTheme){
        switch (theme) {
            case EnumTheme.Dark:
                vant_theme_status.value=this.Dark;
                break;
            case EnumTheme.Light:
                vant_theme_status.value=this.Light;
                break;
        }
    }
}

const tailwind_theme:enum_theme<{Cofe:never},string> = {
    Light:"light",
    Dark: "dark",
    set_theme_f(theme: EnumTheme): void {
        // warn: 事实上, 若扩展新的主题,那么以下代码将出现错误,目前只能处理dark和light
        switch (theme) {
            case EnumTheme.Dark:
                document.documentElement.classList.add('dark');
                break;
            case EnumTheme.Light:
                document.documentElement.classList.remove('dark');
        }
    }
}


//-------------------------------------------------------------
/// 所有主题插件的配置常量,都应该放在这里处理
const set_theme_arr =
    [   vant_theme,
        tailwind_theme,];

// todo 这个状态应该跟随 用户配置文件 ,现在还没有开发那个,所以暂时是EnumTheme.Dark
let theme_status = EnumTheme.Dark;

// 主色状态
// todo 未来扩展
let main_color_status:never;
// 字体状态
// todo 未来扩展
let font_status:never;
//-------------------------------------------------------------



/// 获取当前系统主题
export function get_local_theme():EnumTheme {
    if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        return  EnumTheme.Dark;
    } else {
        return  EnumTheme.Light;
    }
}

/// 设置当前主题
export function set_theme(theme:EnumTheme){
    theme_status=theme;
    set_theme_arr.forEach(v=>v.set_theme_f(theme_status));
}

