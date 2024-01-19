<template>
    <div class="notelist">
        <Toolbar @sendFunction="getFunction" :noteIndex="noteIndex"/>
        <input type="text" v-model="keyword" placeholder="请输入关键词">
        <div class="searchbox" v-show="isShow">
            <ol>
                <li @click="toSearchContent(note)"
                v-for="note in filterNotes" :key="note.id">{{ note.title }}</li>
            </ol>
            <p>{{ result }}</p>
        </div>
        <ol>
            <li class="addnote" @click="addNote">
                <h1>添加新笔记</h1>
            </li>
            <li v-for="note, index in notes" 
                id="notes"
                :key="note.id" 
                @click="toContent(note, index)"
                :class="{clicked: noteIndex === index}"
                :scrollTop="noteIndex">
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

const isShow = ref<boolean>(false)
const keyword = ref<string>('')
const noteIndex = ref<number>(-1) 
const noteStore = useNoteStore()
const {notes} = storeToRefs(noteStore)
const router = useRouter()
let resetIndex: Function = ()=>{}

const filterNotes = computed(()=>{
    return notes.value.filter(note => {
        return note.title.includes(keyword.value) || note.content.includes(keyword.value) || note.tags.some(tag => tag.includes(keyword.value))
    })
})

const result = computed(()=>{
    if (filterNotes.value.length) {
        return `共检索到 ${filterNotes.value.length} 项内容`
    } else {
        return '没有检索到相关内容'
    }
})

watch(keyword, (newValue)=>{
    if (newValue) {
        isShow.value = true
    } else {
        isShow.value = false
    }
})

let onceflag = true
watch(()=>notes.value.length, (newValue, oldValue)=>{
    if (onceflag) {
        onceflag = false
        return
    }
    if (newValue<oldValue) {
        noteIndex.value = -1
        router.push('/empty')
    } else if (newValue>oldValue) {
        noteIndex.value = 0
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

function toSearchContent(note: Note) {
    const index = noteStore.getNoteIndex(note)
    const element = document.getElementsByTagName("li")[index]
    element.scrollIntoView()
    toContent(note, index)
    keyword.value = ''
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
    position: relative;
}
.notelist>input {
    font-size: 1.5rem;
    padding: 0 0.5rem;
    background-color: var(--primiary-color);
    outline: none;
    border: none;
}
.notelist>.searchbox {
    position: absolute;
    top: 10rem;
    left: 0;
    width: 100%;
    background-color: var(--primiary-color);
    transition: all 0.1s;
    box-shadow: 0 0.5rem 1rem 0 rgba(0, 0, 0, 0.1);
}
.notelist>.searchbox>ol {
    display: flex;
    flex-direction: column;
    list-style: none;
}
.notelist>.searchbox>ol>li:first-child {
    border-radius: 0.5rem 0.5rem 0 0;
}
.notelist>.searchbox>ol>li {
    font-size: 1.5rem;
    line-height: 3rem;
    padding: 0 0.5rem;
    border-radius: 0;
}
.notelist>.searchbox>ol>li:hover {
    background-color: var(--click-color);
}
.notelist>.searchbox>p {
    font-size: 1.5rem;
    margin: 0.5rem;
    user-select: none;
}
.notelist>ol {
    display: flex;
    flex-direction: column;
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
    font-weight: normal;
    color: inherit;
    white-space: nowrap; 
    overflow: hidden; 
    text-overflow: ellipsis;
}
.notelist>ol>li>span {
    font-size: 1rem;
    color: inherit;
    margin: 0.5rem 0;
}
.notelist>ol>li>p {
    font-size: 1.5rem;
    color: inherit;
    white-space: nowrap; 
    overflow: hidden; 
    text-overflow: ellipsis;
}
.notelist>ol>li.addnote {
    padding: 1rem 0;
    display: flex;
    flex-direction: row;
    color: var(--confirm-color);
}
.clicked {
    background-color: var(--click-color);
}
</style>


