<template>
    <div class="content">
        <input type="text" v-model="title" placeholder="请输入标题">
        <div class="data">
            <span :class="{saved}">最后保存于：{{ time }}</span>
            <div class="button">
                <div @click="saveSelectedNote">✔</div>
                <div @click="stared = !stared" :class="{stared}">★</div>
                <div>✖</div>
            </div>
        </div>
        <textarea placeholder="请输入内容" v-model="content"></textarea>
        <div class="taglist">
            <span v-for="tag,index in tagList" :key="index" @click="deleteTag(index)">#{{tag}}</span>
            <input class="tag" type="text" v-model="tag" @keyup.enter="addTag" placeholder="请输入标签">
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { useNoteStore } from '../store/note';
import { Note } from '../types';
import { invoke } from '@tauri-apps/api';
import useDialog from '../hooks/useDialog'
 

const title = ref<string>('')
const content = ref<string>('')
const stared = ref<boolean>(false)
const saved = ref<boolean>(false)
const time = ref<string>('')
const tagList = ref<Array<string>>([])
const tag = ref<string>('')
const noteStore = useNoteStore();
const {selectedNote} = storeToRefs(noteStore);
const {showWarningDialog} = useDialog()

watch(selectedNote, ()=>{
    if (selectedNote.value) {
        title.value = selectedNote.value.title;
        content.value = selectedNote.value.content;
        stared.value = selectedNote.value.stared;
        time.value = selectedNote.value.updated_at;
        tagList.value = selectedNote.value.tags;
    }
})
watch([title, content, tagList, stared], ()=>{
    if (saved.value === true) {
        saved.value = false;
    }
},{deep: true})
function addTag() {
    tagList.value.push(tag.value);
    tag.value = '';
}
function deleteTag(index: number) {
    tagList.value.splice(index, 1);
}
async function saveSelectedNote() {
    if (selectedNote.value) {
        if (title.value.trim() !== '' && content.value.trim() !== '') {
            time.value = await invoke('get_time');
            const tempNote: Note = {
                id: selectedNote.value.id,
                title: title.value,
                content: content.value,
                stared: stared.value,
                tags: tagList.value,
                created_at: selectedNote.value.created_at,
                updated_at: time.value,
            }
            noteStore.updateNote(tempNote);
            noteStore.saveNote()
            saved.value = true
        } else {
            showWarningDialog('笔记标题与内容不能为空！')
        }
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
    border: none;
    outline: none;
    font-size: 2rem;
    text-align: center;
    line-height: 6rem;
    font-weight: bold;
    background-color: rgb(40,44,52);
}
.content>.data {
    margin: 0.5rem 0;
    height: 3rem;
    display: flex;
    justify-content: space-between;
}
.content>.data>span {
    font-size: 1.5rem;
    line-height: 3rem;
    margin: 0 1rem;
    user-select: none;
}
.saved {
    color: rgb(152,195,121);
}
.content>.data>.button {
    height: 3rem;
    margin: 0 1rem;
    display: flex;
    align-items: center;
}
.content>.data>.button>div {
    margin: 0 0.5rem;
    width: 2rem;
    height: 3rem;
    font-size: 2rem;
    line-height: 3rem;
    text-align: center;
    user-select: none;
    cursor: pointer;
    transition: all 0.1s;
}
.content>.data>.button>div:first-child {
    color: rgb(152,195,121);
}
.content>.data>.button>div:last-child {
    color: rgb(224,108,117);
}
.content>.data>.button>div:active {
    transform: translateY(-0.5rem);
}
.stared {
    color: gold
}
.content>textarea {
    flex: 1;
    resize: none;
    outline: none;
    border: none;
    background-color: rgb(40,44,52);
    padding: 0.5rem 1rem;
    font-size: 2rem;
    text-align: justify;
    line-height: 3rem;
    box-sizing: border-box;
}
.content>.taglist {
    margin-top: 0.5rem;
    display: flex;
    align-items: center;
    background-color: rgb(33,37,43);
    flex-wrap: wrap;
}
.content>.taglist>span {
    margin: 0 1rem;
    line-height: 3rem;
    font-size: 1.5rem;
    color: rgb(152,195,121);
    user-select: none;
}
.content>.taglist>span:hover {
    cursor: pointer;
    color: rgb(224,108,117);
}
.content>.taglist>input {
    line-height: 3rem;
    font-size: 1.5rem;
    min-width: 30%;
    flex: 1;
    border: none;
    outline: none;
    background-color: rgb(40,44,52);
    padding: 0 0.5rem;
}
</style>