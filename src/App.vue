<script setup lang="ts">
   import { ConfigProvider } from "vant";
   import { vant_theme_status } from "./ts_src/theme";
</script>
<template>
   <ConfigProvider :theme="vant_theme_status" />
   <router-view></router-view> <!--  这里不应该在被修改  -->
</template>