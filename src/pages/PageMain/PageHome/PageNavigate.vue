<script lang = "ts" setup>
    import {Tabbar, TabbarItem,Sticky} from 'vant';
    import Icon from '../../../ts_src/icon';
</script>

<template>

  <router-view></router-view>
  <div style = "height: 45px;width: 100%">
    <!-- 一个填充块,填充导航栏的内容 -->
  </div>
  <Tabbar route style = "height: 45px">
    <TabbarItem name = "nameless_1" replace to = "/home/nameless_1">
      <template #icon = "props">
        <Icon.HeroOutline.ChevronDoubleUpIcon class = "h-6 w-6"></Icon.HeroOutline.ChevronDoubleUpIcon>
      </template>
    </TabbarItem>
    <TabbarItem name = "setting" replace to = "/home/setting">
      <template #icon = "props">
        <Icon.HeroOutline.Cog8ToothIcon class = "h-6 w-6"></Icon.HeroOutline.Cog8ToothIcon>
      </template>
    </TabbarItem>


  </Tabbar>
</template>

<script lang = "ts">
    import {RouteRecordRaw} from "vue-router";
    import ShowSetting from "./PageNavigate/ShowSetting.vue";
    import ShowNameless_1 from "./PageNavigate/ShowNameless_1.vue";

    export const PageNavigateChildren: RouteRecordRaw[] = [
        {
            path: "",
            redirect: "/home/nameless_1"
        },
        {
            path: "nameless_1",
            component: ShowNameless_1
        },
        {
            path: "setting",
            component: ShowSetting
        }
    ];
    export default {}
</script>