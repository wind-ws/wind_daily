<script lang="ts" setup>
   import ComRowTodo from "../../../../components/task/todo/ComRowTodo.vue";
   import { ref, Ref, onMounted } from "vue";
   import { SwipeCell, Button, Cell } from "vant";
   import { Priority, Todo } from "../../../../ts_src/twin/user/user_todo";
   import Sortable from "sortablejs";

   let drag = ref(false);
   let myArray = ref(
      generateItems(30, (i) => ({
         id: i,
         name: {
            id: i,
            is: i % 2 == 0,
            is_visible: true,
            title: "todo " + i,
            priority: (() => {
               switch (i % 5) {
                  case 0:
                     return Priority.Indifferent;
                  case 1:
                     return Priority.Ordinary;
                  case 2:
                     return Priority.Normal;
                  case 3:
                     return Priority.Urgent;
                  case 4:
                     return Priority.Haunted;
               }
            })(),
            create_time: "...",
         } as Todo,
      }))
   );

   function generateItems(count: number, creator: (i: number) => any) {
      const result: any[] = [];
      for (let i = 0; i < count; i++) {
         result.push(creator(i));
      }
      return result;
   }

   const sortable_list = ref<HTMLElement | null>(null);

   onMounted(() => {
      let sort = Sortable.create(sortable_list.value as HTMLElement, {
         delay: 300,
         animation: 150,
         filter: ".ignore-elements-ComRowTodo",
      });
   });
</script>

<template>
   <div class="w-[90%] h-40">
      <!--    筛选器... 要求可以存储和命名-->
      <!--    每一个筛选器都有独立的状态-->
      <!--    通过筛选器 来显示列表-->
      <!--    列表都是通过筛选器生成-->
      <!--    筛选器可以筛选: 标签,是否完成,是否隐藏,根据优先级,设置排序规则和可拖动规则-->
   </div>

   <div
      ref="sortable_list"
      id="abc"
      class="w-[90%] mx-auto">
      <div
         v-for="v in myArray"
         :key="v.id"
         class="mb-4">
         <ComRowTodo :table="v.name"></ComRowTodo>
      </div>
   </div>
</template>
<style scoped>
   .sortable-chosen {
      opacity: 0.5;
   }
</style>
