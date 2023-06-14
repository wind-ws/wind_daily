<script lang="ts" setup></script>

<template>
   <router-view></router-view>
</template>

<script lang="ts">
   import { RouteRecordRaw } from "vue-router";
import ShowChart from "./PageChart/ShowChart.vue";

   export const PageChartChildren: RouteRecordRaw[] = [
      {
         path:"",
         component:ShowChart
      }
   ];
   export default {};
</script>
