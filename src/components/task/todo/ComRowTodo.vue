<script lang = "ts" setup>
    import Icon from "../../../ts_src/icon";
    import {ref} from "vue";
    import type { AnyTouchEvent } from 'any-touch';
    import { useElementSize } from '@vueuse/core';
    import { motion } from 'framer-motion'

    let is = ref(false);
    let s = ref("abc")
    let todo_block=ref<HTMLElement>();


</script>
<template>
  <div ref="todo_block" class = "dark:bg-zinc-900 rounded-xl px-3 py-4 w-full h-min"
       style = "width: 100%"
  >
    <div class="inline-block pr-2">
      <Icon.HeroOutline.CheckCircleIcon class = "inline-block h-6 text-teal-700 "></Icon.HeroOutline.CheckCircleIcon>
    </div>
    <div class="inline-block text-xl">
      123安定{{s}}
    </div>
  </div>
  <motion.div>

  </motion.div>
</template>
