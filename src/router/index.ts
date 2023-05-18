import { RouteRecordRaw,createRouter, createWebHistory, RouterView } from 'vue-router'
import MainPage,{children} from "../pages/PageMain.vue";

//注意了,这种children玩法,在vue中的热更新是无效的
//也就是说,修改path后不会更新
//解决办法是,更新一下这个文件,没错就是 router/index.ts 这个文件
//比如 在注释下添加一个🥵,在下面尽情的添加吧!
/**
 * 🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵
 * 🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵
 * 🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵
 * 🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵🥵
 */
export const routerHistory = createWebHistory();
export const routes:RouteRecordRaw[]=[
    {
        path:"/",
        component:MainPage,
        children:children
    }
];

const router = createRouter({
    history: routerHistory,
    routes:routes
})
export default router;