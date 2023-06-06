<script lang = "ts" setup>
    import ComRowTodo from "../../../../components/task/todo/ComRowTodo.vue";
    import draggable from 'vuedraggable'
    import {ref} from "vue"
    import { SwipeCell ,Button,Cell } from 'vant';
    let drag = ref(false);

    let myArray = ref([
        {
            name:"abc",
        },
        {
            name:"123",
        }
    ])
</script>

<template>
  123
  <div class = "w-[90%] mx-auto ">
    <ComRowTodo></ComRowTodo>
  </div>
  {{myArray}}
  <draggable
          v-model = "myArray"

          group = "people"
          item-key = "id"
          @end = "drag=false"
          @start = "drag=true">
    <template #item = "{element}">
      <div>
<!--        <ComRowTodo class="bg-teal-700" :title="element.name"></ComRowTodo>-->
        <SwipeCell>
          <template #left>
            <Button square type="primary" text="选择" />
          </template>
          <Cell :border="false" title="单元格" value="内容" />
          <template #right>
            <Button square type="danger" text="删除" />
          </template>
        </SwipeCell>
      </div>
    </template>
  </draggable>

</template>
