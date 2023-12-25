<template>
    <div class="notelist">
        <Toolbar/>
        <input type="text" placeholder="请输入关键词">
        <ol>
            <li v-for="note in notes" :key="note.id">
                <h1>{{ note.title }}</h1>
                <span>{{ note.created_at }}</span>
                <span>{{ note.updated_at }}</span>
                <p>{{ note.content }}</p>
            </li>
        </ol>
    </div>
</template>

<script lang="ts" setup>
import Toolbar from './Toolbar.vue'
import { useNoteStore } from '../store/note';

const {notes} = useNoteStore();
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
    border: 0.2rem solid rgb(230,230,230);
    outline: none
}
.notelist>input:focus {
    border: 0.2rem solid rgb(179,225,157);
}
.notelist>ol {
    background-color: aquamarine;
    padding: 2rem;
    overflow: scroll;
}
.notelist>ol>li {
    display: flex;
    flex-direction: column;
}
.notelist>ol>li>h1 {
    font-size: 2rem;
}
.notelist>ol>li>p {
    font-size: 1rem;
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
}
</style>


