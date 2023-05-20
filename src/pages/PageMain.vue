<script lang = "ts" setup>

</script>

<template>
  <router-view></router-view>
</template>

<script lang = "ts">
    import {RouteRecordRaw} from "vue-router";
    import PageHome ,{PageHomeChildren}from "./PageMain/PageHome.vue";
    import PageEntrance,{PageEntranceChildren} from "./PageMain/PageEntrance.vue";

    export const PageMainChildren: RouteRecordRaw[] = [
        {
            path: "",
            component: PageEntrance,
            children:PageEntranceChildren
        },
        {
            path: "/home",
            component: PageHome,
            children:PageHomeChildren
        }
    ];
    // 芜湖!似乎可以这样玩~~太美妙了~🥵🥵🥵
    // 可惜的是,热更新似乎不更新这里...
    // 所以Page修改如果没反应,就手动更新一下
    // 我的意思是 修改一下 router/index.ts 的注释即可
    export default {}
</script>
<!-- 以下是模板,方便复制粘贴
<script lang="ts">
import {RouteRecordRaw} from "vue-router";

export const page_***_children: RouteRecordRaw[] = [

];
export default {}
</script>
-->