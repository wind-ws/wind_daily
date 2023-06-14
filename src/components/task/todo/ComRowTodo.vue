<script lang="ts" setup>
   import { ref } from "vue";
   import { SwipeCell, Cell, Button, Row, Col } from "vant";
   import Icon from "../../../ts_src/icon";
   import user_todo, {
      get_priority_color_done,
      get_priority_color_undone,
      Todo,
   } from "../../../ts_src/twin/user/user_todo";
   /// 双击编辑 title
   /// 左滑出现 隐藏按钮
   /// 右滑出现 删除按钮 和 全面数据编辑(可以编辑除了id以外的全部数据,包括创建时间)
   /// 一般不建议删除,需要保存数据做 数据表 ,建议选择 隐藏

   const props = defineProps<{
      table: Todo;
   }>();
   const id = props.table.id;

   function click_switch_is() {
      //切换is状态,也表示 完成或未完成 的todo
      props.table.is = !props.table.is;
      let is = props.table.is;

      if (is) {
         //done
      } else {
         //undone
      }
      
   }
   function click_conceal() {//隐藏当前id的todo
      props.table.is_visible = !props.table.is_visible;
      user_todo.UpdataTodo.updata_todo(props.table)
      
   }
   function click_change() {
      //修改当前id的todo
   }
   function click_remove() {
      //删除当前id的todo
      //todo 要求弹框确认是否删除,并提示 "建议点击隐藏,以便存储数据生成数据图表"
   }

   const original_height = window.innerHeight; //窗口原高度
   const title_block = ref<HTMLElement | HTMLInputElement | null>(null);
   const dblclick_edit_title_repeat = ref(false); //用于消除重复 双击事件触发监听,其实删除它,也可以正常运行
   function dblclick_edit_title() {
      //双击编辑title ,检测是否脱离聚焦 服务 退出编辑
      if (title_block.value == null || dblclick_edit_title_repeat.value) return;

      dblclick_edit_title_repeat.value = true; //开启禁止双击
      title_block.value.contentEditable = "true"; //设为可编辑
      title_block.value.focus(); //input聚焦

      const range = window.getSelection() as Selection;
      range.selectAllChildren(title_block.value);
      range.collapseToEnd(); //光标移至最后

      window.addEventListener("resize", e_listener_edit); //间接监听键盘弹出和收起

      function e_listener_edit() {
         if (title_block.value == null) return;
         const height = window.innerHeight; //事件触发后的窗口高度
         if (original_height != height) {
            //弹出键盘
            console.log("come~");
         } else {
            //收起键盘
            console.log("oh~yes~");
            dblclick_edit_title_repeat.value = false; //关闭禁止双击
            title_block.value.contentEditable = "false"; //设为不可编辑
            window.removeEventListener("resize", e_listener_edit);
         }
      }
   }
</script>
<template>
   <div
      class="w-full h-full"
      :style="{ opacity: table.is ? 0.6 : 1 }">
      <SwipeCell
         :right-width="80"
         :left-width="140">
         <template #left>
            <div
               class="flex h-full ignore-elements-ComRowTodo"
               style="width: 140px">
               <Row
                  gutter="3"
                  align="center"
                  class="h-full w-full">
                  <Col
                     span="10"
                     style="height: max-content">
                     <div
                        class="w-12 h-12 mx-auto flex border-2 border-red-900 rounded-full active:bg-red-900"
                        @click="click_remove">
                        <Icon.HeroOutline.TrashIcon
                           class="text-red-700 h-6 mx-auto self-center" />
                     </div>
                  </Col>
               </Row>
               <Row
                  gutter="3"
                  align="center"
                  class="h-full w-full">
                  <Col
                     span="10"
                     style="height: max-content">
                     <div
                        class="w-12 h-12 mx-auto flex border-2 border-cyan-900 rounded-full active:bg-cyan-900"
                        @click="click_change">
                        <Icon.HeroOutline.WrenchScrewdriverIcon
                           class="text-cyan-700 h-6 mx-auto self-center" />
                     </div>
                  </Col>
               </Row>
            </div>
         </template>
         <div
            class="h-full w-full rounded-xl px-3 py-4 bg-zinc-900"
            style="min-height: 50px"
            @dblclick="dblclick_edit_title">
            <Row
               gutter="3"
               align="center">
               <Col
                  span="3"
                  justify="space-between">
                  <div
                     class="self-center w-full h-full border-teal-700"
                     @click="click_switch_is">
                     <Icon.HeroSolid.StopIcon
                        v-if="table.is"
                        class="inline-block h-7"
                        :style="{
                           color: get_priority_color_done(table.priority),
                        }" />
                     <Icon.HeroOutline.StopIcon
                        v-else
                        class="inline-block h-7"
                        :style="{
                           color: get_priority_color_undone(table.priority),
                        }" />
                  </div>
               </Col>
               <Col span="21">
                  <div
                     ref="title_block"
                     class="unselectable"
                     style="word-break: break-all; outline: none">
                     {{ props.table.title }}
                  </div>
               </Col>
            </Row>
         </div>
         <template #right>
            <div
               class="flex h-full ignore-elements-ComRowTodo"
               style="width: 100px">
               <Row
                  gutter="3"
                  align="center"
                  class="h-full w-full">
                  <Col
                     span="24"
                     style="height: max-content">
                     <div
                        class="w-12 h-12 mx-auto flex border-2 border-zinc-700 rounded-full active:bg-zinc-700"
                        @click="click_conceal">
                        <Icon.HeroOutline.EyeSlashIcon
                           class="text-zinc-500 h-6 mx-auto self-center" />
                     </div>
                  </Col>
               </Row>
            </div>
         </template>
      </SwipeCell>
   </div>
</template>
<style scoped>
   .ignore-elements-ComRowTodo {
   } /*专门用于防止 被拖动,不可添加任何css属性*/

   .unselectable {
      /*拒绝文字选择*/
      user-select: none;
      -webkit-touch-callout: none;
      -webkit-user-select: none;
      -khtml-user-select: none;
      -moz-user-select: none;
      -ms-user-select: none;
   }
</style>
