<template>
    <div class="toolbar">
        <div class="button" @click="router.push('/content')">笔记</div>
        <div class="button" @click="router.push('/todo')">待办</div>
        <div class="button" @click="router.push('/cover')">封面</div>
        <div class="button">设置</div>
    </div>
</template>
  
<script lang="ts" setup>
import { useRouter } from 'vue-router'

const router = useRouter()

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
    font-size: 2rem;
    line-height: 4rem;
    text-align: center;
    user-select: none;
    transition: all 0.1s;
    cursor: pointer;
}
.button:hover {
    border: 0.2rem solid rgb(180,180,180);
}
.button:active {
    background-color: rgb(180,180,180);
    color: rgb(40,44,52);
}
</style>