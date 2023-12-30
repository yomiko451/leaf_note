<template>
    <div class="content">
        <input type="text" v-model="selectedNote.title" placeholder="请输入标题">
        <div class="data">
            <span :class="selectedNote.saved?'saved':'unsaved'">{{ saveInfo }}</span>
            <div class="button">
                <div @click="saveSelectedNote">✔</div>
                <div @click="noteStore.changeNoteStarred" :class="selectedNote.starred?'starred':''">★</div>
                <div @click="deleteSelectedNote">✖</div>
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
import { invoke } from '@tauri-apps/api';
import useDialog from '../hooks/useDialog'

const tag = ref<string>('')
const noteStore = useNoteStore();
const {selectedNote, selectedNoteIndex} = storeToRefs(noteStore);
const {showWarningDialog, showAskDialog} = useDialog()

const saveInfo = computed(()=>{
    if (selectedNote.value.saved) {
        return '最后保存于：' + selectedNote.value.updated_at
    } else {
        if (selectedNote.value.updated_at) {
            return '笔记内容已修改，请及时保存！' 
        } else {
            return '新建笔记尚未保存，请及时保存！'
        }
    }
})

watch([
    ()=>selectedNote.value.title, 
    ()=>selectedNote.value.content,
    ()=>selectedNote.value.starred,
    ()=>selectedNote.value.tags,
], ()=>{
    noteStore.changeNoteToUnsaved()
},{deep: true})

function addTag() {
    if (tag.value.trim()) {
        noteStore.addTag(tag.value);
        tag.value = '';
    } else {
        showWarningDialog('标签不能为空！')
    }
}

async function saveSelectedNote() {
    if (selectedNote.value.title.trim() && selectedNote.value.content.trim()) {
        const time: string = await invoke('get_time');
        noteStore.updateNoteTime(time)
        noteStore.changeNoteToSaved()
        noteStore.saveNote()
    } else {
        showWarningDialog('笔记标题与内容不能为空！')
    }
}

async function deleteSelectedNote() {
        let res = await showAskDialog(`确定删除笔记 ${selectedNote.value.title} ？`)
        if (res) {
            res = await invoke('check_note_exist', {item: selectedNote.value})
            if (res) {
                noteStore.deleteLocalNote(selectedNote.value, selectedNoteIndex.value);
            } else {
                noteStore.deleteNote(selectedNoteIndex.value)
            }
            noteStore.reset()
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
.unsaved {
    color: rgb(224,108,117);
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
    transform: scale(1.5);
}
.starred {
    color: rgb(152,195,121);
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
    height: 3rem;
    font-size: 1.5rem;
    min-width: 30%;
    flex: 1;
    border: none;
    outline: none;
    background-color: rgb(40,44,52);
    padding: 0 0.5rem;
}
.content>.taglist>div {
    line-height: 3rem;
    font-size: 2rem;
    color: rgb(152,195,121);
    margin: 0 0.5rem;
    transition: all 0.1s;
    user-select: none;
    cursor: pointer;
}
.content>.taglist>div:active {
    transform: scale(1.5);
}
</style>