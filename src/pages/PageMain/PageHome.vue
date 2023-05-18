<script lang="ts" setup>
import {Button, Cell, CellGroup,Switch} from 'vant';
import {EnumTheme, set_theme} from "../../ts_src/theme";
import {ref} from "vue";

const checked = ref(true);
function fff(value: any) {
    if(value)
        set_theme(EnumTheme.Dark)
    else
        set_theme(EnumTheme.Light)

}

</script>

<template>

    <CellGroup>
        <Cell title="单元格" value="内容"/>
        <Cell label="描述信息" title="单元格" value="内容"/>
    </CellGroup>
    <Button type="primary">123</Button>
    <Switch v-model="checked" @change="fff"></Switch>
    <div class="h-20 w-20 bg-amber-200 dark:bg-blue-600">
        123
    </div>
</template>

<style scoped>

</style>