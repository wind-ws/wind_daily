<script lang="ts" setup>
   import { Tabbar, TabbarItem, Sticky } from "vant";
   import Icon from "../../../ts_src/icon";
</script>

<template>
   <router-view></router-view>
   <div style="height: 45px; width: 100%">
      <!-- 一个填充块,填充导航栏的内容 -->
   </div>
   <Tabbar
      route
      style="height: 45px">
      <TabbarItem
         name="todo"
         replace
         to="/home/todo">
         <template #icon="props">
            <Icon.HeroOutline.CheckBadgeIcon
               class="h-6 w-6"></Icon.HeroOutline.CheckBadgeIcon>
         </template>
      </TabbarItem>
      <TabbarItem
         name="show"
         replace
         to="/home/show">
         <template #icon="props">
            <Icon.HeroOutline.RectangleGroupIcon
               class="h-6 w-6"></Icon.HeroOutline.RectangleGroupIcon>
         </template>
      </TabbarItem>
      <TabbarItem
         name="chart"
         replace
         to="/home/chart">
         <template #icon="props">
            <Icon.HeroOutline.ChartBarIcon
               class="h-6 w-6"></Icon.HeroOutline.ChartBarIcon>
         </template>
      </TabbarItem>
      <TabbarItem
         name="setting"
         replace
         to="/home/setting">
         <template #icon="props">
            <Icon.HeroOutline.Cog8ToothIcon
               class="h-6 w-6"></Icon.HeroOutline.Cog8ToothIcon>
         </template>
      </TabbarItem>
      <!-- 小部件显示页 -->
      <!-- 数据图看板页 -->
      <!-- 设置页-->
   </Tabbar>
</template>

<script lang="ts">
   import { RouteRecordRaw } from "vue-router";
   import ShowSetting from "./PageNavigate/ShowSetting.vue";
   import ShowNameless_1 from "./PageNavigate/ShowNameless_1.vue";

   export const PageNavigateChildren: RouteRecordRaw[] = [
      {
         path: "",
         redirect: "/home/todo",
      },
      {
         path: "todo", // todo页
         component: ShowNameless_1,
      },
      {
         path: "setting", //设置页
         component: ShowSetting,
      },
   ];
   export default {};
</script>
