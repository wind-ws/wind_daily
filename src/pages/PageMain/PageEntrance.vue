<!--
   App的入口页面
   用于 添加用户数据文件夹,创建用户数据文件夹
   若 App配置文件中 配置默认启动用户,则可以直接进入 PageHome
-->
<script lang="ts" setup></script>

<template>
   <router-view></router-view>
</template>

<script lang="ts">
   import { RouteRecordRaw } from "vue-router";
   import ShowEntrance from "./PageEntrance/ShowEntrance.vue";

   export const PageEntranceChildren: RouteRecordRaw[] = [
      {
         path: "",
         component: ShowEntrance,
      },
   ];
   export default {};
</script>
