<script lang = "ts" setup>
    import Icon from "../../../ts_src/icon";
    import {ref} from "vue";
    import type {AnyTouchEvent} from 'any-touch';
    import anime from 'animejs/lib/anime.es.js';
    /// 双击编辑 title
    /// 左滑出现 隐藏按钮
    /// 右滑出现 删除按钮 和 全面数据编辑(可以编辑除了id以外的全部数据,包括创建时间)
    /// 一般不建议删除,需要保存数据做 数据表 ,建议选择 隐藏

    const props = defineProps<{
        id: number,
        is:boolean,
        title:string
    }>();

    const todo_block = ref<HTMLElement | null>(null)
    const delete_block = ref<HTMLElement | null>(null)

    let right_fold = {
        unfold_x: -60,//展开的x尺寸
        fold_x: 0,//折叠的x尺寸
        end_x_is_unfold: -50,//触摸离开后,若x距离满足(x<end_x_is_unfold)则展开
        x: ref(0),
        is_fold: ref<boolean>(true),
        unfold() {
            this.is_fold.value = false;
            this.x.value = this.unfold_x;
            anime({
                targets: todo_block.value,
                translateX: this.x.value,
                duration: 1000
            });
            anime({
                targets: delete_block.value,
                translateX: 0,
                width: 50,
                // height:"100%",
                duration: 1000,
            });
        },
        fold() {
            this.is_fold.value = true;
            this.x.value = this.fold_x;
            anime({
                targets: todo_block.value,
                translateX: this.x.value,
                duration: 1000
            });
            anime({
                targets: delete_block.value,
                translateX: 0,
                width: 0,
                // height:"0%",
                duration: 1000,
            });
        },
        panright(e: AnyTouchEvent) {
            this.x.value = this.x.value + e.deltaX;
            anime({
                targets: todo_block.value,
                translateX: this.x.value,
                duration: 0
            });
            this.move_delete_block();
        },
        panleft(e: AnyTouchEvent) {
            this.x.value = this.x.value + e.deltaX;
            anime({
                targets: todo_block.value,
                translateX: this.x.value,
                duration: 0
            });
            this.move_delete_block();
        },
        panend(e: AnyTouchEvent) {
            if (this.x.value < this.end_x_is_unfold) {
                this.unfold();
            } else {
                this.fold();
            }
        },
        move_delete_block() {
            if (this.x.value <= -20 && this.x.value > -70) {
                anime({
                    targets: delete_block.value,
                    translateX: 0,
                    // height:((Math.abs(this.x.value) -20)*2)+"%",
                    width: Math.abs(this.x.value) - 20,
                    duration: 0
                });
            } else if (this.x.value <= -70) {
                anime({
                    targets: delete_block.value,
                    translateX: this.x.value + 70,
                    // height:"100%",
                    width: 50,
                    duration: 0
                });
            }
        }
    }


</script>
<template>
  <div class = "w-full h-full inline-block relative">
    <div ref = "todo_block"
         v-touch
         class = "inline-block
                  h-full w-full
                  rounded-xl px-3 py-4
                  dark:bg-zinc-900
                  "
         @panend = "right_fold.panend"
         @panleft = "right_fold.panleft"
         @panright = "right_fold.panright">
      <div class = "inline-block pr-2 w-[10%]">
        <Icon.HeroOutline.CheckCircleIcon
                class = "h-6 text-teal-700"></Icon.HeroOutline.CheckCircleIcon>
      </div>
      <div class = "inline-block text-xl h-full w-[90%] "
           style = "word-break: break-all;">
        {{title}}
      </div>
    </div>
    <div ref = "delete_block"
         class = "inline-block absolute bottom-0 right-0
                h-full w-0 rounded-xl bg-red-800"
         style = "display: flex;align-items: center;">
      <Icon.HeroOutline.XCircleIcon
              class = "w-[75%] text-black" style = "margin: 0 auto"></Icon.HeroOutline.XCircleIcon>
    </div>
  </div>

</template>
