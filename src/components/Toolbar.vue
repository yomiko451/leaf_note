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
    background-color: rgb(40,44,52);
    display: flex;
    align-items: center;
    justify-content: space-evenly;
}

.button {
    height: 4rem;
    width: 8rem;
    box-sizing: border-box;
    font-size: 2rem;
    line-height: 3.6rem;
    text-align: center;
    user-select: none;
    transition: all 0.1s;
}
.button:hover {
    border: 0.2rem solid rgb(180,180,180);
}
.button:active {
    background-color: rgb(180,180,180);
    color: rgb(40,44,52);
}
</style>