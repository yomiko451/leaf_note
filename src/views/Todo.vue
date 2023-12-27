<template>
    <div class="todo">
        <h1>待办事项 10 已完成 5</h1>
        <div class="options">
            <input type="text" placeholder="请输入待办事项组名称">
            <div>添加</div>
            <div>删除</div>
        </div>
        <ol>
            <li>
                <p>待办事项</p>
            </li>
        </ol>
    </div>
</template>

<style scoped>
.todo {
    display: grid;
    height: 100%;
    width: 100%;
    grid-template-rows: 6rem 3rem 1fr;
    gap: 0.5rem;
}
.todo>h1 {
    height: 6rem;
    line-height: 6rem;
    font-size: 2rem;
    text-align: center;
    background-color: rgb(40,44,52);
    font-weight: normal;
}
.todo>.options {
    display: flex;
}
.todo>.options>div {
    height: 3rem;
    width: 6rem;
    font-size: 1.5rem;
    line-height: 3rem;
    text-align: center;
    user-select: none;
    transition: all 0.1s;
    cursor: pointer;
    margin-left: 0.5rem;
    background-color: rgb(40,44,52);
}
.todo>.options>:nth-child(2) {
    color: rgb(152,195,121);
}
.todo>.options>:last-child {
    color: rgb(224,108,117);
}
.todo>.options>div:active {
    background-color: rgba(180,180,180,0.25);
}
.todo>.options>input {
    flex: 1;
    padding: 0 0.5rem;
    background-color: rgb(40,44,52);
    outline: none;
    transition: all 0.1s;
    border: none;
    box-sizing: border-box;
}
.todo>input:focus {
    border: 0.2rem solid rgb(180,180,180);
}
.todo>ol {
    display: flex;
    flex-direction: column;
    justify-content: center;
    overflow: scroll;
    background-color: rgb(40,44,52);
}
</style>

<script lang="ts" setup>

</script>