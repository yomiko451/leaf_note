<template>
    <div class="subwindow">
        <p>请输入城市名称</p>
        <input type="text" v-model="text">
        <div>
            <div @click="confirm(text)">确定</div>
            <div @click="subwindowClose()">取消</div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref, inject } from 'vue';
import { useConfigStore } from '../store/config';
import useDialog from '../hooks/useDialog';

const text = ref<string>('')
const configStore = useConfigStore();
const {showSuccessDialog, showWarningDialog} = useDialog();
const subwindowClose = inject('SWC') as Function ;

async function confirm(text: string) {
    if (text) {
        configStore.updateWeather(text);
        await showSuccessDialog('天气信息更新成功！')
        subwindowClose()
    } else {
        await showWarningDialog('城市名称不能为空！')
    }
}
</script>

<style scoped>
.subwindow {
    top: 30rem;
    left: 40rem;
    transform: translate(-50%, -50%);
    width: 32rem;
    height: 12rem;
    padding: 1rem 0; 
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    position: fixed;
    background-color: var(--secondry-color);
}
.subwindow>p {
    font-size: 2rem;
}
.subwindow>input {
    line-height: 4rem;
    font-size: 2rem;
    text-align: center;
    border: none;
    outline: none;
    background-color: var(--primiary-color);
}
.subwindow>div {
    width: 20rem;
    display: flex;
    justify-content: space-between;
}
.subwindow>div>div {
    width: 8rem;
    text-align: center;
    cursor: pointer;
    line-height: 4rem;
    background-color: var(--primiary-color);
    transition: all 0.1s;
}
.subwindow>div>:first-child {
    color: var(--confirm-color);
}
.subwindow>div>:last-child {
    color: var(--warning-color);
}
.subwindow>div>div:active {
    background-color: var(--click-color);
}
</style>