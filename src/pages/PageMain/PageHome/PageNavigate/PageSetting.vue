<script lang="ts" setup></script>

<template>
   <router-view></router-view>
</template>

<script lang="ts">
   import { RouteRecordRaw } from "vue-router";
import ShowSetting from "./PageSetting/ShowSetting.vue";

   export const PageSettingChildren: RouteRecordRaw[] = [
      {
         path:"",
         component:ShowSetting
      }
   ];
   export default {};
</script>
