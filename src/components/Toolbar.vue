<template>
    <div class="toolbar">
        <div :class="{selected: selectedIndex === 0}" @click="toContent">笔记</div>
        <div :class="{selected: selectedIndex === 1}" @click="toTodo">待办</div>
        <div :class="{selected: selectedIndex === 2}" @click="toCover">封面</div>
        <div :class="{selected: selectedIndex === 3}">设置</div>
    </div>
</template>
  
<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { useNoteStore } from '../store/note';
import { ref } from 'vue'
import { storeToRefs } from 'pinia';

const selectedIndex = ref<number>(2)
const router = useRouter()
const noteStore = useNoteStore()
const {notes} = storeToRefs(noteStore)

function toContent() {
    if (notes.value.length === 0) {
        router.push({
            path: '/empty',
            query: {
                text: '单击左侧按钮添加新笔记'
            }
        })
    } else {
        router.push('/content')
    }
    selectedIndex.value = 0
}

async function toTodo() {
    router.push('/todo')
    selectedIndex.value = 1
}

async function toCover() {
    router.push('/cover')
    selectedIndex.value = 2
}

function resetIndex() {
    selectedIndex.value = 0
}

const emit = defineEmits(['sendFunction'])
emit('sendFunction', resetIndex)
</script>
  
<style scoped>
.toolbar {
    background-color: rgb(40,44,52);
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
    border: 0.2rem solid rgb(180,180,180);
}
.toolbar>div:active {
    background-color: rgb(180,180,180);
    color: rgb(40,44,52);
}
.selected {
    background-color: rgb(180,180,180);
    color: rgb(40,44,52);
}
</style>