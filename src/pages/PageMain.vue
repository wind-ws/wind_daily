
<script setup lang="ts">


</script>


<template>
    <router-view></router-view>
</template>

<style scoped>

</style>

<script lang="ts">
import {RouteRecordRaw} from "vue-router";
import PageHome from "./PageMain/PageHome.vue";
export const children : RouteRecordRaw[]= [
    {
        path:"/",
        component:PageHome
    }
];
// 芜湖!似乎可以这样玩~~太美妙了~🥵🥵🥵
// 可惜的是,热更新似乎不更新这里...
// 所以Page修改如果没反应,就手动更新一下
// 我的意思是 修改一下 router/index.ts 的注释即可
export default {}
</script>