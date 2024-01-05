<template>
    <ol class="contextmenu" v-show="isShow" id="contextmenu">
        <li>剪切</li>
        <li>复制</li>
        <li>粘贴</li>
        <li>全选</li>
        <li>删除</li>
    </ol>
</template>

<script lang="ts" setup>
import {ref} from 'vue'

const isShow = ref<boolean>(false)
document.addEventListener('click', (e)=>{
    if (e.button === 0) {
        isShow.value=false
    }
})
document.oncontextmenu = (e) => {
    e.preventDefault();
    let element = e.target as HTMLElement
    if (element.tagName === 'INPUT' || element.tagName === 'TEXTAREA') {
        const menu = document.getElementById('contextmenu')
        if (menu) {
            menu.style.top = e.clientY + 'px'
            menu.style.left = e.clientX + 'px'
        }
        isShow.value = true
    }
  }
</script>

<style scoped>
.contextmenu {
    position: absolute;
    width: 10rem;
    background-color: var(--primiary-color);
    list-style: none;
    box-shadow: 0 0.5rem 1rem 0 rgba(0, 0, 0, 0.1);
    padding: 0.5rem;
}
.contextmenu>li:first-child {
    border-radius: 0.5rem 0.5rem 0 0;
}
.contextmenu>li:last-child {
    border-radius: 0 0 0.5rem 0.5rem;
}
.contextmenu>li {
    font-size: 1.5rem;
    line-height: 3rem;
    padding: 0 0.5rem;
    cursor: pointer;
    transition: all 0.1s;
    border-radius: 0;
}
.contextmenu>li:hover {
    background-color: var(--click-color);
}
</style>