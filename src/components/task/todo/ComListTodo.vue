<script setup lang="ts">
   import { onMounted, ref,Ref } from "vue";
   import ComRowTodo from "./ComRowTodo.vue";
   import { AddTodo, GetTodo, Todo } from "../../../ts_src/twin/user/user_todo";
   import Sortable from "sortablejs";

   // 需要传入 筛选机
   // 会根据 筛选机 来加载状态和 todo列表
   const props = defineProps<{
      e:{
         // 添加器 添加todo时 我们使用这个函数去增加我们的列表
         // 需要重新定义函数规则,去覆盖,传入的函数必然没有任何功能
         // 父组件触发子组件,奇怪的用法,不知道行不行
         refresh_todo_list:(todo:AddTodo.AddTodo)=>void,
      },

   }>(); 

   const sortable_list = ref<HTMLElement | null>(null);

   props.e.refresh_todo_list = (todo:AddTodo.AddTodo)=>{
      refresh_todo_list();
   };

   onMounted(() => {
      let sort = Sortable.create(sortable_list.value as HTMLElement, {
         delay: 300,
         animation: 150,
         filter: ".ignore-elements-ComRowTodo",
      });
   });
   

   // ! 以下是临时的 TodoList 状态,暂时先发布0.1后写筛选机,状态由筛选机保持

   const todo_list = ref<Todo[]>([]);
   function remove_byid(index:number) {
      todo_list.value.splice(index,1);
   }
   function refresh_todo_list(){
      GetTodo.get_all_todo()
         .then((v) => {
            v.reverse();
            todo_list.value = v;
         }).catch((e) => {});
   }
   refresh_todo_list();
</script>

<template>

   <div ref="sortable_list" >
      <div v-for="v,k in todo_list"
         :key="v.id" class="mb-4">
         <ComRowTodo :row="v" @remove_byid="remove_byid(k)"></ComRowTodo>
      </div>
   </div>
</template>
<style scoped>
   .sortable-chosen {
      opacity: 0.5;
   }
</style>
