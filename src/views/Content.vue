<template>
    <div class="content">
        <input type="text" v-model="selectedNote.title" placeholder="请输入标题">
        <div class="data">
            <span class="saved">{{ saveInfo }}</span>
            <div @click="noteStore.changeNoteLocked" :class="selectedNote.locked?'':'locked'">{{ lockstate }}</div>
            <div @click.self="deleteSelectedNote">删除
                <div v-if="selectedNote.locked" class="disabled">--</div>
            </div>
        </div>
        <textarea placeholder="请输入内容" v-model="selectedNote.content"></textarea>
        <div class="taglist">
            <span v-for="tag,index in selectedNote.tags" :key="index" @click="noteStore.deleteTag(index)">#{{tag}}</span>
            <input class="tag" type="text" v-model="tag" @keyup.enter="addTag" placeholder="请输入标签">
            <div @click="addTag">✚</div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref, watch, computed} from 'vue';
import { storeToRefs } from 'pinia';
import { useNoteStore } from '../store/note';
import useDialog from '../hooks/useDialog'

const tag = ref<string>('')
const noteStore = useNoteStore();
const {selectedNote} = storeToRefs(noteStore);
const {showWarningDialog, showAskDialog} = useDialog()

const saveInfo = computed(()=>{
    return '最后保存于：' + selectedNote.value.updated_at
})

const lockstate = computed(()=>{
    return selectedNote.value.locked?'解锁':'锁定'
})

watch([
    ()=>selectedNote.value.id,
    ()=>selectedNote.value.title, 
    ()=>selectedNote.value.content,
    ()=>selectedNote.value.tags,
], (newValue, oldValue)=>{
    if (newValue[0] === oldValue[0]) {
        debounce(noteStore.saveNote)
    }
}, {deep: true})

let timerId: number | null = null
function debounce(func: Function) {
    if (timerId) {
        clearTimeout(timerId)
    }
    timerId = setTimeout(() => {
        func()
    }, 1000);
}

function addTag() {
    if (tag.value.trim()) {
        noteStore.addTag(tag.value);
        tag.value = '';
    } else {
        showWarningDialog('标签不能为空！')
    }
}

async function deleteSelectedNote() {
        let res = await showAskDialog(`确定删除笔记 ${selectedNote.value.title} ？`)
        if (res) {
            noteStore.deleteNote(selectedNote.value);
        } 
}
</script>

<style scoped>
.content {
    display: flex;
    height: 100%;
    width: 100%;
    flex-direction: column;
}
.content>input {
    height: 6rem;
    padding: 0 1rem;
    border: none;
    outline: none;
    font-size: 2rem;
    text-align: center;
    line-height: 6rem;
    font-weight: bold;
    background-color: var(--primiary-color);
    white-space: nowrap; 
    overflow: hidden; 
    text-overflow: ellipsis;
}
.content>.data {
    margin: 0.5rem 0;
    height: 3rem;
    display: flex;
}
.content>.data>span {
    font-size: 1.5rem;
    line-height: 3rem;
    margin-left: 1rem;
    user-select: none;
    flex: 1;
}
.saved {
    color: var(--confirm-color);
}
.content>.data>div {
    height: 3rem;
    width: 6rem;
    font-size: 1.5rem;
    line-height: 3rem;
    text-align: center;
    user-select: none;
    transition: all 0.1s;
    cursor: pointer;
    margin-left: 0.5rem;
    background-color: var(--primiary-color);
}
.content>.data>div:last-child {
    color: var(--warning-color);
    position: relative;
}
.disabled {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: var(--primiary-color);
    cursor: not-allowed;
}
.content>.data>div:active {
    background-color: var(--click-color);
}
.locked {
    color: var(--confirm-color);
}
.content>textarea {
    flex: 1;
    resize: none;
    outline: none;
    border: none;
    background-color: var(--primiary-color);
    padding: 0.5rem 1rem;
    font-size: 2rem;
    font-family: inherit;
    text-align: justify;
    line-height: 3.5rem;
    box-sizing: border-box;
    word-break: break-all;
}
.content>.taglist {
    margin-top: 0.5rem;
    display: flex;
    align-items: center;
    background-color: var(--secondry-color);
    flex-wrap: wrap;
}
.content>.taglist>span {
    margin: 0 1rem;
    line-height: 3rem;
    font-size: 1.5rem;
    color: var(--confirm-color);
    user-select: none;
}
.content>.taglist>span:hover {
    cursor: pointer;
    color: var(--warning-color);
}
.content>.taglist>input {
    height: 3rem;
    font-size: 1.5rem;
    min-width: 30%;
    flex: 1;
    border: none;
    outline: none;
    background-color: var(--primiary-color);
    padding: 0 0.5rem;
}
.content>.taglist>div {
    line-height: 3rem;
    font-size: 2rem;
    color: var(--confirm-color);
    margin: 0 0.5rem;
    transition: all 0.1s;
    user-select: none;
    cursor: pointer;
}
.content>.taglist>div:active {
    transform: scale(1.5);
}
</style>