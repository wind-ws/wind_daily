<script lang = "ts" setup>

</script>

<template>
  <router-view></router-view>
</template>

<script lang = "ts">
    import {RouteRecordRaw} from "vue-router";

    export const {{name}}Children: RouteRecordRaw[] = [];
    export default {}
</script>