<template>
    <div class="setting">
        <Subwindow v-if="subwindowShow"/>
        <div class="customize" @click="subwindowShow = true">
            <p>天气查询对应城市</p>
            <div>{{ config?.city }}</div>
        </div>
        <div class="customize" @click="configStore.updateCoverFilter">
            <p>封面图片黑白滤镜</p>
            <div>{{ config?.cover_filter? '开启' : '关闭' }}</div>
        </div>
        <div class="customize">
            <p>窗口大小</p>
            <div>正常</div>
        </div>
        <div class="customize">
            <p>显示字体</p>
            <div>正常</div>
        </div>
        <div class="customize">
            <p>显示字号</p>
            <div>正常</div>
        </div>
        <div class="folder">打开笔记保存文件夹</div>
        <div class="folder">打开待办保存文件夹</div>
        <div class="folder">打开封面保存文件夹</div>
</div>
</template>

<script lang="ts" setup>
import { useConfigStore } from '../store/config';
import { storeToRefs } from 'pinia';
import Subwindow from '../components/Subwindow.vue'
import { ref, provide } from 'vue';


const subwindowShow = ref(false);
const configStore = useConfigStore();
const {config} = storeToRefs(configStore);

function subwindowClose() {
    subwindowShow.value = false;
}
provide('SWC', subwindowClose)

</script>


<style scoped>
.setting {
    width: 100%;
    height: 100%;
    background-color: rgb(40,44,52);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    user-select: none;
}
.setting>.customize {
    width: 30rem;
    height: 4rem;
    margin: 0.5rem 0;
    padding: 0 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: rgb(180,180,180);
    cursor: pointer;
    box-sizing: border-box;
    border: 0.2rem solid rgb(40,44,52);
    transition: all 0.1s;
}
.setting>.customize>p {
    line-height: 3.6rem;
    color: inherit;
}
.setting>.customize>div {
    line-height: 3.6rem;
    color: inherit;
}
.setting>.customize:hover {
    border: 0.2rem solid rgb(180,180,180);
}
.setting>.customize:active {
    background-color: rgb(180,180,180);
    border: 0.2rem solid rgb(40,44,52);
    color: rgb(40,44,52);
}
.setting>.folder {
    height: 4rem;
    margin: 0.5rem 0;
    padding: 0 1rem;
    line-height: 3.6rem;
    text-align: center;
    cursor: pointer;
    box-sizing: border-box;
    transition: all 0.1s;
    border: 0.2rem solid rgb(40,44,52);
}
.setting>.folder:hover {
    border: 0.2rem solid rgb(180,180,180);
}
.setting>.folder:active {
    background-color: rgb(180,180,180);
    border: 0.2rem solid rgb(40,44,52);
    color: rgb(40,44,52);
}
</style>