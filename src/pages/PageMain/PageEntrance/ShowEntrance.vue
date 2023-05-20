<script lang = "ts" setup>
    import {EllipsisVerticalIcon, PlayIcon} from "@heroicons/vue/24/outline"
    import {Popup} from 'vant';
    import {ref} from "vue";

    const show_popup = ref(false);


</script>

<template>
  <Popup v-model:show = "show_popup"
         class = "h-1/5 p-2"
         round style = " width: 70%">
  
  </Popup>
  <div class = "mt-28 w-full text-center text-5xl">
    <span class = " bg-clip-text dark:text-transparent
                    bg-gradient-to-r
                    from-transparent from-5%
                    via-cyan-400
                    to-teal-500
                ">Wind </span>
    <span class = " bg-clip-text dark:text-transparent
                    bg-gradient-to-r
                    from-teal-500 from-5%
                    via-teal-400 via-45%
                    to-95%
                    ">Daily</span>
  </div>
  <div class = "mt-48 h-60 w-full">
    <div class = "h-16 w-3/5"
         style = "margin: auto">
      <div class = "h-full w-[73%] text-center
                  border-solid border-4 rounded-l-lg border-teal-600"
           style = "float: left;line-height:55px ">
        <PlayIcon class = "w-10 text-teal-500 inline"></PlayIcon>

      </div>
      <div class = "h-full w-[25%] text-center
                  border-solid border-4 rounded-r-lg border-teal-600"
           style = "float: right;line-height:55px"
           @click = "show_popup=!show_popup">
        <EllipsisVerticalIcon class = "inline w-10 text-teal-500"></EllipsisVerticalIcon>
      </div>
    </div>
  </div>
    <!--  又是讨厌Css的一天~🫥  -->
</template>
