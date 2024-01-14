<template>
    <ol class="contextmenu" v-show="isShow" id="contextmenu">
        <li @click="cutText" @mousedown="getNoFocus">剪切</li>
        <li @click="copyText" @mousedown="getNoFocus">复制</li>
        <li @click="pasteText" @mousedown="getNoFocus">粘贴</li>
        <li @click="selectAll" @mousedown="getNoFocus">全选</li>
        <li @click="deleteText" @mousedown="getNoFocus">删除</li>
    </ol>
</template>

<script lang="ts" setup>
import {ref} from 'vue'
import {readText, writeText} from '@tauri-apps/api/clipboard'


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
        const height = document.documentElement.clientHeight
        const width = document.documentElement.clientWidth
        const rem = parseInt(document.documentElement.style.fontSize)
        if (menu) {
            menu.style.top = ''
            menu.style.left = ''
            menu.style.right = ''
            menu.style.bottom = ''
            if (e.clientX + (10*rem) >= width) {
                menu.style.right = (width - e.clientX) + 'px'
            } else {
                menu.style.left = e.clientX + 'px'
            }
            if (e.clientY + (16*rem) >= height) {
                menu.style.bottom = (height - e.clientY) + 'px'
            } else {
                menu.style.top = e.clientY + 'px'
            }
        }
        isShow.value = true
    }
}

async function copyText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart !== null && selectelement.selectionEnd !== null) {
        const text = selectelement.value.substring(selectelement.selectionStart, selectelement.selectionEnd)
        await writeText(text)
    }
}

async function cutText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const text = selectelement.value.substring(selectelement.selectionStart, selectelement.selectionEnd)
        await writeText(text)
        const temp = selectelement.selectionStart
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + selectelement.value.substring(selectelement.selectionEnd)
        selectelement.selectionStart = selectelement.selectionEnd = temp
    }
}

async function pasteText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const text = await readText()
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + text + selectelement.value.substring(selectelement.selectionEnd)
    }
}

function selectAll() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        selectelement.selectionStart = 0
        selectelement.selectionEnd = selectelement.value.length
    }
}

function deleteText() {
    const selectelement = document.activeElement as HTMLInputElement
    if (selectelement.selectionStart!== null && selectelement.selectionEnd!== null) {
        const temp = selectelement.selectionStart
        selectelement.value = selectelement.value.substring(0, selectelement.selectionStart) + selectelement.value.substring(selectelement.selectionEnd)
        selectelement.selectionStart = selectelement.selectionEnd = temp
    }
}

function getNoFocus(e: MouseEvent) {
    e.preventDefault()
}
</script>

<style scoped>
.contextmenu {
    position: absolute;
    width: 10rem;
    background-color: var(--primiary-color);
    list-style: none;
    box-shadow: 0 0 1rem 0.25rem var(--context-color);
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
    user-select: none;
}
.contextmenu>li:hover {
    background-color: var(--click-color);
}
</style>