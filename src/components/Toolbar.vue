<template>
    <div class="toolbar">
        <div :class="{selected: selectedIndex === 0}" @click="toContent">笔记</div>
        <div :class="{selected: selectedIndex === 1}" @click="toTodo">待办</div>
        <div :class="{selected: selectedIndex === 2}" @click="toCover">封面</div>
        <div :class="{selected: selectedIndex === 3}" @click="toSetting">设置</div>
    </div>
</template>
  
<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { ref } from 'vue'
import {Note} from '../types'

const selectedIndex = ref<number>(2)
const router = useRouter()


function toContent() {
    selectedIndex.value = 0
    if (parentData.filterNotes.length) {
        router.push('/content')
    } else {
        router.push({
            path: '/empty',
            query: {
                text: '暂无内容'
            }
        })
    }
}

function toTodo() {
    router.push('/todo')
    selectedIndex.value = 1
}

function toCover() {
    router.push('/cover')
    selectedIndex.value = 2
}

function toSetting() {
    router.push('/setting')
    selectedIndex.value = 3
}

function resetIndex() {
    selectedIndex.value = 0
}

const emit = defineEmits(['sendFunction'])
emit('sendFunction', resetIndex)

const parentData = defineProps<{filterNotes: Array<Note>}>()
</script>
  
<style scoped>
.toolbar {
    background-color: var(--primiary-color);
    display: flex;
    align-items: center;
    justify-content: space-evenly;
}
.toolbar>div {
    height: 4rem;
    width: 8rem;
    font-size: 2rem;
    line-height: 4rem;
    text-align: center;
    user-select: none;
    transition: all 0.1s;
    cursor: pointer;
}
.toolbar>div:hover {
    background-color: var(--click-color);    
}
.selected {
    background-color: var(--click-color); 
}
</style>