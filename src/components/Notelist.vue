<template>
    <div class="notelist">
        <Toolbar @sendFunction="getFunction" :noteIndex="noteIndex"/>
        <input type="text" v-model="keyword" placeholder="请输入关键词">
        <ol>
            <li @click="addNote">
                <h1>添加新笔记</h1>
                <span>添加标记可以让笔记变得更加醒目</span>
                <p>笔记内容支持实时自动保存</p>
            </li>
            <li v-for="note, index in filterNotes" 
                :key="note.id" 
                @click="toContent(note, index)"
                :class="{clicked: noteIndex === index}">
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
import { ref, computed, watch } from 'vue'
import { storeToRefs } from 'pinia'

const keyword = ref<string>('')
const noteIndex = ref<number>(-2) // -2: 删除笔记后或笔记数量为空时，展示空白页，-1: 有笔记，但不选中任何笔记，0: 选中新增笔记
const noteStore = useNoteStore()
const {notes} = storeToRefs(noteStore)
const router = useRouter()
let resetIndex: Function = ()=>{}

const filterNotes = computed(()=>{
    return notes.value.filter(note => {
        return note.title.includes(keyword.value) || note.content.includes(keyword.value)
    })
})

// 这个逻辑太丑陋了！！！！！
let onceflag = true
watch([()=>notes.value.length, keyword], (newValue, oldValue)=>{
    if (onceflag) {
        onceflag = false
        return
    }
    resetIndex()
    if (newValue[0] < oldValue[0] || newValue[0]===0) { // 删除笔记或笔记数量为空，展示空白页
        noteIndex.value = -2
        router.push({
            path: '/empty',
            query: {
                text: '暂无内容'
            }
        })
    } else if(newValue[0] > oldValue[0]) { // 新增笔记
        noteIndex.value = 0
    } else { // 不选中任何笔记，保持内容页不变
        noteIndex.value = -1
    }
})

function getFunction(func: Function) {
    resetIndex = func
}

function toContent(note: Note, index: number) {
    noteStore.updateSelectedNote(note)
    router.push('/content')
    noteIndex.value = index
    resetIndex()
}

function addNote() {
    noteStore.addNote()
    keyword.value = ''
    router.push('/content')
    resetIndex()
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


