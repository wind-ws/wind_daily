<script lang = "ts" setup>
    import ComRowTodo from "../../../../components/task/todo/ComRowTodo.vue";
    import {ref} from "vue"
    import { SwipeCell ,Button,Cell } from 'vant';
    import { Container, Draggable } from "vue3-smooth-dnd";

    let drag = ref(false);

    let myArray = ref([
        {
            name:"abc",
        },
        {
            name:"123",
        }
    ])
    function applyDrag(arr:any, dragResult:any) {
        const { removedIndex, addedIndex, payload } = dragResult;
        console.error("aa")
        if (removedIndex === null && addedIndex === null) return arr;

        const result = [...arr];
        let itemToAdd = payload;

        if (removedIndex !== null) {
            itemToAdd = result.splice(removedIndex, 1)[0];
        }

        if (addedIndex !== null) {
            result.splice(addedIndex, 0, itemToAdd);
        }

        return result;
    };

    function generateItems(count:number, creator:(i:number)=>any){
        const result:any[] = [];
        for (let i = 0; i < count; i++) {
            result.push(creator(i));
        }
        return result;
    };

    let items = ref(generateItems(15, (i:number) => ({ id: i, data: "Draggable " + i })));
    function onDrop(dropResult:any) {
        console.log(dropResult)
        items.value = applyDrag(items.value, dropResult);
    };
</script>

<template>
<!--  123-->
  <div class = "w-[90%] mx-auto ">
<!--    <ComRowTodo></ComRowTodo>-->
  </div>

<!-- 只要发生拖动后,就无法滚动了... -->
  <div class = "w-[90%] mx-auto " style="height: 100vh;overflow-y: auto">
    <Container orientation="vertical" @drop="onDrop" lock-axis="y" :drag-begin-delay="800">
      <Draggable v-for="(item, i) in items" :key="item.id">
        <div class="mb-4">
          <ComRowTodo :title="item.data"></ComRowTodo>
        </div>
      </Draggable>
    </Container>
  </div>


</template>
