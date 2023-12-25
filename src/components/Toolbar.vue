<template>
    <div class="toolbar">
        <div class="button" @click="addNote">新键</div>
        <div class="button">删除</div>
        <div class="button">星标</div>
        <div class="button">复制</div>
    </div>
</template>
  
<script lang="ts" setup>
import { useNoteStore } from '../store/note';

const {addNote} = useNoteStore();
</script>
  
<style scoped>
.toolbar {
    background-color: pink;
    display: flex;
    align-items: center;
    justify-content: space-evenly;
}

.button {
    height: 4rem;
    width: 8rem;
    background-color: rgb(103, 194, 58);
    color: white;
    font-size: 2rem;
    line-height: 4rem;
    text-align: center;
    user-select: none;
}
.button:active {
    background-color: rgb(83, 164, 48);
}
</style>