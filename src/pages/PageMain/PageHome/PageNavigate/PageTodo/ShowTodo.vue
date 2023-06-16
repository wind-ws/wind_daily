<script lang="ts" setup>
   import ComRowTodo from "../../../../../components/task/todo/ComRowTodo.vue";
   import { ref, Ref, onMounted } from "vue";
   import { SwipeCell, Button, Cell } from "vant";
   import { Priority, Todo } from "../../../../../ts_src/twin/user/user_todo";
   import Sortable from "sortablejs";
import ComListTodo from "../../../../../components/task/todo/ComListTodo.vue";



</script>

<template>
   <div class="w-[90%] h-40">
      <Button @click="()=>{
         
      }">添加todo</Button>
   </div>

   <div
      ref="sortable_list"
      id="abc"
      class="w-[90%] mx-auto">
      <!-- <ComListTodo></ComListTodo> -->
   </div>
</template>
