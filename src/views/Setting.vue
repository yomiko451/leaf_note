<template>
    <div class="setting">
        <Subwindow v-if="subwindowShow" :title="title" :func="func" />
        <div class="customize" @click="setWeather">
            <p>天气查询对应城市</p>
            <div>{{ config?.city }}</div>
        </div>
        <div class="customize" @click="configStore.updateCoverFilter">
            <p>封面图片黑白滤镜</p>
            <div>{{ config?.cover_filter? '开启' : '关闭' }}</div>
        </div>
        <div class="customize" @click="configStore.updateUiSize">
            <p>缩放比例</p>
            <div>{{ size }}</div>
        </div>
        <div class="customize" @click="configStore.updateUiStyle">
            <p>主题颜色</p>
            <div>{{ config?.ui_style?'深色':'浅色' }}</div>
        </div>
        <div class="customize" @click="setFont">
            <p>显示字体</p>
            <div>{{ config?.font_family }}</div>
        </div>
        <div class="folder" @click="invoke('open_folder', {code: 0})">打开笔记保存文件夹</div>
        <div class="folder" @click="invoke('open_folder', {code: 1})">打开待办保存文件夹</div>
        <div class="folder" @click="invoke('open_folder', {code: 2})">打开封面保存文件夹</div>
</div>
</template>

<script lang="ts" setup>
import { useConfigStore } from '../store/config';
import { storeToRefs } from 'pinia';
import Subwindow from '../components/Subwindow.vue'
import { ref, provide, computed } from 'vue';
import {invoke} from '@tauri-apps/api/tauri'

const title = ref<string>('');
const func = ref<Function>(()=>{});
const subwindowShow = ref(false);
const configStore = useConfigStore();
const {config} = storeToRefs(configStore);

const size = computed(() => {
    if (config.value) {
        switch (config.value.ui_scale) {
            case 0:
                return '0.5x';
            case 1:
                return '标准';
            case 2:
                return '1.5x';
        } 
    }
})

function subwindowClose() {
    subwindowShow.value = false;
}
provide('SWC', subwindowClose)

function setWeather() {
    title.value = '请输入城市名称';
    func.value = useConfigStore().updateWeather;
    subwindowShow.value = true
}

function setFont() {
    title.value = '请输入字体名称';
    func.value = useConfigStore().updateFontFamily;
    subwindowShow.value = true
}
</script>


<style scoped>
.setting {
    width: 100%;
    height: 100%;
    background-color: var(--primiary-color);
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
    margin: 1rem 0;
    padding: 0 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: var(--letter-color);
    cursor: pointer;
    box-sizing: border-box;
    transition: all 0.1s;
}
.setting>.customize>p {
    line-height: 4rem;
    color: inherit;
}
.setting>.customize>div {
    line-height: 4rem;
    color: inherit;
}
.setting>.customize:hover {
    background-color: var(--click-color);
}
.setting>.folder {
    height: 4rem;
    margin: 1rem 0;
    padding: 0 1rem;
    line-height: 4rem;
    text-align: center;
    cursor: pointer;
    box-sizing: border-box;
    transition: all 0.1s;
}
.setting>.folder:hover {
    background-color: var(--click-color);
}
</style>