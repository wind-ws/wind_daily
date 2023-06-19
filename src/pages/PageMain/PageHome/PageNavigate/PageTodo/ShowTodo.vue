<script lang="ts" setup>
   import ComRowTodo from "../../../../../components/task/todo/ComRowTodo.vue";
   import { ref, Ref, onMounted } from "vue";
   import { SwipeCell, Button, Cell, Field } from "vant";
   import {
      AddTodo,
      Priority,
      Todo,
      get_priority_color_undone,
      priority_arr,
   } from "../../../../../ts_src/twin/user/user_todo";
   import Sortable from "sortablejs";
   import ComListTodo from "../../../../../components/task/todo/ComListTodo.vue";

   const title = ref("");
   const priority = ref<Priority>(Priority.Normal);
   const priority_select = ref(false);
   // 用于 父组件 触发 子组件
   const e_add_todo = ref((_:AddTodo.AddTodo)=>{});
   const e=ref({
      add_todo:e_add_todo
   })

   function add_todo() {
      if (title.value == "") return;
      const todo = { 
         title: title.value,
         priority: priority.value,
      };
      AddTodo.add_todo(todo);
      e.value.add_todo(todo);

      title.value = "";
   }
</script>

<template>
   <div class="w-[90%] h-40 mx-auto py-10">
      <Field
         v-model="title"
         class="rounded-xl"
         rows="1"
         autosize
         type="textarea"
         placeholder="todo title"
         label-width="26">
         <template #label>
            <div
               class="h-full w-full flex relative"
               style="z-index: 999">
               <div class="self-center flex justify-start gap-x-4 w-[200px]">
                  <div
                     class="border-[3px] rounded-full h-[20px] w-[20px]"
                     :style="{
                        borderColor: get_priority_color_undone(priority),
                     }"
                     @click="priority_select = !priority_select"></div>

                  <div
                     v-if="priority_select"
                     v-for="v in priority_arr().filter((v) => v != priority)"
                     class="border-[3px] rounded-full h-[20px] w-[20px]"
                     :style="{
                        borderColor: get_priority_color_undone(v),
                     }"
                     @click="
                        () => {
                           priority = v;
                           priority_select = !priority_select;
                        }
                     "></div>
               </div>
            </div>
         </template>
         <template #button>
            <div
               class="text-zinc-600 active:text-zinc-400 unselectable"
               @click="add_todo">
               Add
            </div>
         </template>
      </Field>
   </div>

   <div
      ref="sortable_list"
      id="abc"
      class="w-[90%] mx-auto">
      <ComListTodo :e="e"></ComListTodo>
   </div>
</template>
