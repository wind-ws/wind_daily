import { createApp } from "vue";
import App from "./App.vue";
import {app_init_after, app_init_before} from "./ts_src/init";



app_init_before();

createApp(App).mount("#app");

app_init_after();
