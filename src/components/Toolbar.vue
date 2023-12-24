<template>
    <div class="toolbar">
        <div class="button">新键</div>
        <div class="button">删除</div>
        <div class="button">星标</div>
        <div class="button">复制</div>
        <div class="button">复制</div>
    </div>
</template>
  
<script lang="ts" setup></script>
  
<style scoped>
.toolbar {
    grid-row: 1/2;
    grid-column: 1/3;
    background-color: pink;
    display: flex;
    align-items: center;
    justify-content: space-evenly;
}

.button {
    height: 4rem;
    width: 10rem;
    background-color: rgb(103, 194, 58);
    color: white;
    font-size: 2rem;
    line-height: 4rem;
    text-align: center;
}

</style>