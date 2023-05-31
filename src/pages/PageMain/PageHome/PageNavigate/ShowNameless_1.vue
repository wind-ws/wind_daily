<script setup lang = "ts">
    import {InsertValue} from "../../../../ts_src/twin/user/user_example";
    import {ref} from "vue";
    let a = ref<InsertValue.Example>({
        id: 1,
        name: 'abc',
        age: 33,
        address:"home",
        // salary:1000
    });
    InsertValue.insert_value(a.value);
</script>

<template>
nameless

</template>
