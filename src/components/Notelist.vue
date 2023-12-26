<template>
    <div class="notelist">
        <Toolbar/>
        <input type="text" placeholder="请输入关键词">
        <ol>
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
import { Note } from '../types';
import { useRouter } from 'vue-router';
import { useNoteStore } from '../store/note';


const noteStore = useNoteStore();
const router = useRouter();
function toContent(note: Note, index: number) {
    noteStore.$patch({
        selectedNote: note,
        selectedNoteIndex: index
    })
    router.push('/content');
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
    padding: 0 0.5rem;
    background-color: rgb(40,44,52);
    outline: none;
    transition: all 0.1s;
    border: none;
    box-sizing: border-box;
}
.notelist>input:focus {
    border: 0.2rem solid rgb(180,180,180);
}
.notelist>ol {
    background-color: rgb(40,44,52);
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
    background-color: rgb(180,180,180);
    color: rgb(40,44,52);
}
.notelist>ol>li>h1 {
    font-size: 2rem;
    color: inherit;
}
.notelist>ol>li>span {
    font-size: 1rem;
    color: inherit;
}
.notelist>ol>li>p {
    font-size: 1.5rem;
    color: inherit;
}
.clicked {
    background-color: rgb(180,180,180);
    color: rgb(40,44,52);
}
</style>


