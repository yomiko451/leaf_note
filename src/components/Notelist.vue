<template>
    <div class="notelist">
        <Toolbar @sendFunction="getFunction"/>
        <input type="text" placeholder="请输入关键词">
        <ol>
            <li @click="addNote">
                <h1>添加新笔记</h1>
                <span>这里会记录笔记的创建和修改时间</span>
                <p>不要忘记手动保存笔记内容！</p>
            </li>
            <li v-for="note, index in noteStore.notes" 
                :key="note.id" 
                @click="toContent(note, index)"
                :class="{clicked: noteStore.selectedNoteIndex === index}">
                <h1>{{ note.title? note.title : '无标题' }}</h1>
                <span>{{ note.created_at }} -- {{ note.updated_at }}</span>
                <p>{{ note.content? note.content : '无内容' }}</p>
            </li>
        </ol>
    </div>
</template>

<script lang="ts" setup>
import Toolbar from './Toolbar.vue'
import { Note } from '../types'
import { useRouter } from 'vue-router'
import { useNoteStore } from '../store/note'
import { ref } from 'vue'

const noteStore = useNoteStore()
const router = useRouter()
const resetIndex = ref<Function>(()=>{})

function getFunction(func: Function) {
    resetIndex.value = func
}

function toContent(note: Note, index: number) {
    noteStore.updateSelectedNote(note, index)
    router.push('/content')
    resetIndex.value()
}

function addNote() {
    noteStore.addNote()
    router.push('/content')
    resetIndex.value()
}
</script>

<style scoped>
.notelist {
    min-height: 0;
    display: grid;
    grid-template-rows: 6rem 3rem 1fr;
    gap: 0.5rem;
}
.notelist>input {
    font-size: 1.5rem;
    padding: 0 0.5rem;
    background-color: var(--primiary-color);
    outline: none;
    transition: all 0.1s;
    border: none;
}
.notelist>ol {
    background-color: var(--primiary-color);
    padding: 0.5rem 1rem;
    overflow: scroll;
}
.notelist>ol>li {
    display: flex;
    flex-direction: column;
    justify-content: center;
    cursor: pointer;
    transition: all 0.1s;
    user-select: none;
    padding: 0.5rem 1rem;
    margin: 0.5rem 0;
}
.notelist>ol>li:hover {
    background-color: var(--click-color);
}
.notelist>ol>li>h1 {
    font-size: 2rem;
    color: inherit;
    white-space: nowrap; 
    overflow: hidden; 
    text-overflow: ellipsis;
}
.notelist>ol>li>span {
    font-size: 1rem;
    color: inherit;
}
.notelist>ol>li>p {
    font-size: 1.5rem;
    color: inherit;
    white-space: nowrap; 
    overflow: hidden; 
    text-overflow: ellipsis;
}
.clicked {
    background-color: var(--click-color);
}
</style>


