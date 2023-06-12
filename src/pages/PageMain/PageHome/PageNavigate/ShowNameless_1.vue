<script lang = "ts" setup>
    import ComRowTodo from "../../../../components/task/todo/ComRowTodo.vue";
    import {ref,Ref,onMounted} from "vue"
    import { SwipeCell ,Button,Cell } from 'vant';
    import Sortable from 'sortablejs';

    let drag = ref(false);
    let myArray = ref(generateItems(20,(i)=>{return {id:i,name:"todo "+i}}))
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

    const sortable_list = ref< HTMLElement | null>(null)

    onMounted(() => {
        let sort = Sortable.create(sortable_list.value as HTMLElement,{
            delay:500,
            animation: 150,
        });
    })
</script>

<template>

  <div class = "w-[90%] h-40 ">
  </div>


  <div ref="sortable_list" id="abc" class = "w-[90%] mx-auto " >
    <div v-for="v in myArray" :key="v.id"  class="mb-4">
      <ComRowTodo :title="v.name" ></ComRowTodo>
    </div>
  </div>

</template>
