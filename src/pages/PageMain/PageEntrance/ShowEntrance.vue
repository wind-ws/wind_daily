<script lang = "ts" setup>
    import {EllipsisVerticalIcon, PlayIcon} from "@heroicons/vue/24/outline"
    import {Button, Field, Popup, showToast} from 'vant';
    import {ref} from "vue";
    import {CreateNewUser} from "../../../ts_src/twin/app/app_config";

    const show_popup = ref(false);


    const show_popup_create_new_user = ref(false);
    const user_name = ref("");

    function create_new_user() {
        if (user_name.value == "") {
            showToast("不能为空");
            return;
        }
        if (!/^[\u4E00-\u9FA5a-zA-Z_]+$/u.test(user_name.value)) {
            showToast("用户名只能是字母和中文和下划线");
            return;
        }
        CreateNewUser.create_new_user(user_name.value)
            .catch(v => showToast(v));
        
    }

</script>

<template>
  <Popup v-model:show = "show_popup"
         class = "h-auto p-2"
         round style = " width: 60%">
    <div class = "h-16 w-full text-center leading-[4rem]"
         @click = "show_popup_create_new_user=!show_popup_create_new_user">
      <span class = "inline-block h-full text-xl">创建新用户</span>
    </div>
  </Popup>
  <Popup v-model:show = "show_popup_create_new_user"
         class = "h-auto p-2"
         round style = " width: 70%">
    <Field v-model = "user_name" input-align = "center" placeholder = "请输入用户名" />
    <Button class = "w-full" color = "#009488" hairline plain
            @click = "create_new_user()">创建新用户
    </Button>
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
           style = "float: left;line-height:55px "
      >
        <PlayIcon class = "inline w-10 text-teal-500"></PlayIcon>

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
