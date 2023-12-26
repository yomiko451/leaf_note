<template>
    <div class="content">
        <input type="text" v-model="title" placeholder="请输入标题">
        <div class="data">
            <span>{{ selectedNote?.created_at }}</span>
            <span>{{ selectedNote?.updated_at }}</span>
            <span>{{ selectedNote?.stared }}</span>
        </div>
        <textarea placeholder="请输入内容" v-model="content"></textarea>
        <div class="taglist">
            <span v-for="tag,index in tagList" :key="index">#{{tag}}</span>
            <input class="tag" type="text" v-model="tag" @keyup.enter="addTag" placeholder="请输入标签">
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { useNoteStore } from '../store/note';

const title = ref<string>('')
const content = ref<string>('')
const tagList = ref<Array<string>>([])
const tag = ref<string>('')
const noteStore = useNoteStore();
const {selectedNote} = storeToRefs(noteStore);

watch(selectedNote, ()=>{
    if (selectedNote.value) {
        title.value = selectedNote.value.title;
        content.value = selectedNote.value.content;
        tagList.value = selectedNote.value.tags;
    }
})
watch(content, ()=>{
    if (content.value.includes('# ')) {
        content.value = content.value.replace(/# /g, '标签：')
    }
})
function addTag() {
    tagList.value.push(tag.value);
    tag.value = '';
}
</script>

<style scoped>
.content {
    display: grid;
    height: 100%;
    width: 100%;
    grid-template-rows: 6rem 3rem 1fr 3rem;
    gap: 0.5rem;
}
.content>input {
    border: none;
    outline: none;
    font-size: 2rem;
    text-align: center;
    line-height: 6rem;
    background-color: rgb(40,44,52);
}
.content>.data {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 0.5rem;
}
.content>.data>span {
    font-size: 1.5rem;
    text-align: center;
    line-height: 2rem;
}
.content>textarea {
    resize: none;
    outline: none;
    border: none;
    background-color: rgb(40,44,52);
    padding: 0.5rem;
    font-size: 2rem;
    text-align: justify;
    line-height: 3rem;
    box-sizing: border-box;
}
.content>.taglist {
    display: flex;
    align-items: center;
    background-color: rgb(33,37,43);
}
.content>.taglist>span {
    margin: 0 0.5rem;
    line-height: 3rem;
    font-size: 1.5rem;
    color: rgb(152,195,121);
}
.content>.taglist>input {
    margin: 0;
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