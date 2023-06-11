<script lang = "ts" setup>
    import {SwipeCell,Cell,Button,Row,Col} from 'vant';
    import Icon from "../../../ts_src/icon";
    /// 双击编辑 title
    /// 左滑出现 隐藏按钮
    /// 右滑出现 删除按钮 和 全面数据编辑(可以编辑除了id以外的全部数据,包括创建时间)
    /// 一般不建议删除,需要保存数据做 数据表 ,建议选择 隐藏

    const props = defineProps<{
        _id: number,
        is: boolean,
        title: string
    }>();

    // const todo_block = ref<HTMLElement | null>(null)
    // const delete_block = ref<HTMLElement | null>(null)
    //
    // let right_fold = {
    //     unfold_x: -60,//展开的x尺寸
    //     fold_x: 0,//折叠的x尺寸
    //     end_x_is_unfold: -50,//触摸离开后,若x距离满足(x<end_x_is_unfold)则展开
    //     x: ref(0),
    //     is_fold: ref<boolean>(true),
    //     unfold() {
    //         this.is_fold.value = false;
    //         this.x.value = this.unfold_x;
    //         anime({
    //             targets: todo_block.value,
    //             translateX: this.x.value,
    //             duration: 1000
    //         });
    //         anime({
    //             targets: delete_block.value,
    //             translateX: 0,
    //             width: 50,
    //             // height:"100%",
    //             duration: 1000,
    //         });
    //     },
    //     fold() {
    //         this.is_fold.value = true;
    //         this.x.value = this.fold_x;
    //         anime({
    //             targets: todo_block.value,
    //             translateX: this.x.value,
    //             duration: 1000
    //         });
    //         anime({
    //             targets: delete_block.value,
    //             translateX: 0,
    //             width: 0,
    //             // height:"0%",
    //             duration: 1000,
    //         });
    //     },
    //     panright(e: AnyTouchEvent) {
    //         this.x.value = this.x.value + e.deltaX;
    //         anime({
    //             targets: todo_block.value,
    //             translateX: this.x.value,
    //             duration: 0
    //         });
    //         this.move_delete_block();
    //     },
    //     panleft(e: AnyTouchEvent) {
    //         this.x.value = this.x.value + e.deltaX;
    //         anime({
    //             targets: todo_block.value,
    //             translateX: this.x.value,
    //             duration: 0
    //         });
    //         this.move_delete_block();
    //     },
    //     panend(e: AnyTouchEvent) {
    //         if (this.x.value < this.end_x_is_unfold) {
    //             this.unfold();
    //         } else {
    //             this.fold();
    //         }
    //     },
    //     move_delete_block() {
    //         if (this.x.value <= -20 && this.x.value > -70) {
    //             anime({
    //                 targets: delete_block.value,
    //                 translateX: 0,
    //                 // height:((Math.abs(this.x.value) -20)*2)+"%",
    //                 width: Math.abs(this.x.value) - 20,
    //                 duration: 0
    //             });
    //         } else if (this.x.value <= -70) {
    //             anime({
    //                 targets: delete_block.value,
    //                 translateX: this.x.value + 70,
    //                 // height:"100%",
    //                 width: 50,
    //                 duration: 0
    //             });
    //         }
    //     }
    // }


</script>
<template>
  <div class = "w-full h-full">
    <SwipeCell :right-width="80" :left-width="140">
      <template #left>
        <div class="flex h-full" style="width: 140px">
          <Row gutter="3" align="center" class="h-full w-full">
            <Col span="10" style="height: max-content" >
              <div class="w-12 h-12 mx-auto flex
                        border-2 border-red-900 rounded-full">
                <Icon.HeroOutline.TrashIcon
                        class="text-red-700 h-6 mx-auto self-center"/>
              </div>
            </Col>
          </Row>
          <Row gutter="3" align="center" class="h-full w-full">
            <Col span="10" style="height: max-content" >
              <div class="w-12 h-12 mx-auto flex
                        border-2 border-cyan-900 rounded-full">
                <Icon.HeroOutline.WrenchScrewdriverIcon
                        class="text-cyan-700 h-6 mx-auto self-center"/>
              </div>
            </Col>
          </Row>
        </div>
      </template>
      <div class = "h-full w-full
                    rounded-xl px-3 py-4
                    bg-zinc-900" style = "min-height: 50px;">
        <Row gutter="3" align="center">
          <Col span="3" justify="space-between" >
            <div class="self-center w-full h-full border-teal-700">
              <Icon.HeroOutline.StopIcon class="inline-block h-7 text-zinc-600 "/>
<!--              <Icon.HeroSolid.StopIcon class="inline-block h-7 text-zinc-600 "/>-->
            </div>
          </Col>
          <Col span="21">
            <div style = "word-break: break-all;">
              {{props.title}}
            </div>
          </Col>
        </Row>
      </div>
      <template #right>
        <div class="flex h-full" style="width: 100px">
          <Row gutter="3" align="center" class="h-full w-full">
            <Col span="24" style="height: max-content" >
              <div class="w-12 h-12 mx-auto flex
                        border-2 border-zinc-700 rounded-full">
                <Icon.HeroOutline.EyeSlashIcon
                        class="text-zinc-500 h-6 mx-auto self-center"/>
              </div>
            </Col>
          </Row>
        </div>
      </template>
    </SwipeCell>
  </div>

  <!--  <div class = "w-full h-full inline-block relative">-->
  <!--    <div ref = "todo_block"-->
  <!--         v-touch-->
  <!--         class = "inline-block-->
  <!--                  h-full w-full-->
  <!--                  rounded-xl px-3 py-4-->
  <!--                  dark:bg-zinc-900-->
  <!--                  "-->
  <!--         @panend = "right_fold.panend"-->
  <!--         @panleft = "right_fold.panleft"-->
  <!--         @panright = "right_fold.panright">-->
  <!--      <div class = "inline-block pr-2 w-[10%]">-->
  <!--        <Icon.HeroOutline.CheckCircleIcon-->
  <!--                class = "h-6 text-teal-700"></Icon.HeroOutline.CheckCircleIcon>-->
  <!--      </div>-->
  <!--      <div class = "inline-block text-xl h-full w-[90%] "-->
  <!--           style = "word-break: break-all;">-->
  <!--        {{title}}-->
  <!--      </div>-->
  <!--    </div>-->
  <!--    <div ref = "delete_block"-->
  <!--         class = "inline-block absolute bottom-0 right-0-->
  <!--                h-full w-0 rounded-xl bg-red-800"-->
  <!--         style = "display: flex;align-items: center;">-->
  <!--      <Icon.HeroOutline.XCircleIcon-->
  <!--              class = "w-[75%] text-black" style = "margin: 0 auto"></Icon.HeroOutline.XCircleIcon>-->
  <!--    </div>-->
  <!--  </div>-->

</template>
