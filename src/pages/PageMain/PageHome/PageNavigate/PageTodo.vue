<script lang="ts" setup></script>

<template>
   <router-view></router-view>
</template>

<script lang="ts">
   import { RouteRecordRaw } from "vue-router";
   import ShowTodo from "./PageTodo/ShowTodo.vue";

   export const PageTodoChildren: RouteRecordRaw[] = [
      {
         path: "",
         component: ShowTodo,
      },
   ];
   export default {};
</script>
