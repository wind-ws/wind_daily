//import vue
import { createApp } from "vue";
import App from "./App.vue";
//import 其他代码
import {app_init_after, app_init_before} from "./ts_src/init";
//import 插件
import router from "./router";
    /*@ts-ignore*/
import v_touch from "@any-touch/vue3";

//import css
import "../css/main_style.css";
//-------------------------------
app_init_before();
//-------------------------------
const plugins:any[] = [
    router,v_touch
];
//-------------------------------
export const app = createApp(App);
plugins.forEach(a=>app.use(a));
app.mount("#app");
//-------------------------------
app_init_after();
//-------------------------------
