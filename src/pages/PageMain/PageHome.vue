<!--
    这个页面需要 提供 用户配置即可开启
    根据用户配置加载 App核心页面
-->
<script lang="ts" setup>
</script>

<template>
  <router-view></router-view>
</template>

<script lang="ts">
    import {RouteRecordRaw} from "vue-router";

    export const PageHomeChildren: RouteRecordRaw[] = [

    ];
    export default {}
</script>