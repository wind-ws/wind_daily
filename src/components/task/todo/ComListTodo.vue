<script setup lang="ts">
   import { onMounted, ref } from "vue";
   import ComRowTodo from "./ComRowTodo.vue";
   import user_todo, { Todo } from "../../../ts_src/twin/user/user_todo";
import Sortable from "sortablejs";

   // 需要传入 筛选机
   // 会根据 筛选机 来加载状态和 todo列表

   const sortable_list = ref<HTMLElement | null>(null);
   onMounted(() => {
      let sort = Sortable.create(sortable_list.value as HTMLElement, {
         delay: 300,
         animation: 150,
         filter: ".ignore-elements-ComRowTodo",
      });
   });

   // ! 以下是临时的 TodoList 状态,暂时先发布0.1后写筛选机,状态由筛选机保持

   const todo_list =ref<Todo[]>([]);
   user_todo.GetTodo.get_all_todo()
      .then(v=>{
         todo_list.value=v;
      }).catch(e=>{});


</script>

<template>
   <div ref="sortable_list"
      v-for="v in todo_list"
      :key="v.id"
      class="mb-4">
      <ComRowTodo :row="v"></ComRowTodo>
   </div>
</template>
<style scoped>
   .sortable-chosen {
      opacity: 0.5;
   }
</style>