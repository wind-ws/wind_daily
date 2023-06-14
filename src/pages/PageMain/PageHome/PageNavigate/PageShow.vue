<script lang="ts" setup></script>

<template>
   <router-view></router-view>
</template>

<script lang="ts">
   import { RouteRecordRaw } from "vue-router";
import ShowShow from "./PageShow/ShowShow.vue";

   export const PageShowChildren: RouteRecordRaw[] = [
      {
         path:"",
         component: ShowShow
      }
   ];
   export default {};
</script>
