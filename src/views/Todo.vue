<template>
    <div class="todo">
        <h1>待办事项 10 已完成 5</h1>
        <div class="options">
            <input @keyup.enter="addNoteList" v-model="title" type="text" placeholder="请输入事项组名称">
            <div @click="addNoteList">添加</div>
        </div>
        <ol class="groups">
            <li v-for="todoList, index in todoListArr" :key="todoList.id">
                <div class="title">
                    <p>{{ todoList.title }}</p>
                    <p>{{ todoList.content.filter(i => i.completed).length + ' / ' + todoList.content.length }}</p>
                    <div>删除</div>
                </div>
                <span>{{ todoList.created_at }}</span>
                <div class="add">
                    <input @keyup.enter="addTodo(index)" v-model="todo" type="text" placeholder="请输入待办事项">
                    <div @click="addTodo(index)">✚</div>
                </div>
                <ol class="items">
                    <li v-for="item, subindex in todoList.content" :key="item.id">
                        <p :class="item.completed? 'completed' : ''">{{ item.content }}</p>
                        <div @click="stateChange(item, index, subindex)" :class="item.completed? '' : 'completed'">✔</div>
                        <div>✖</div>
                    </li>
                </ol>
            </li>
        </ol>
    </div>
</template>

<script lang="ts" setup>
import useDialog from '../hooks/useDialog'
import { storeToRefs } from 'pinia';
import { useTodoStore } from '../store/todo';
import { Todo } from '../types'
import {ref} from 'vue'

const todoStore = useTodoStore()
const {todoListArr} = storeToRefs(todoStore)
const {showWarningDialog} = useDialog()
const title = ref<string>('')
const todo = ref<string>('')

function addNoteList() {
    if (title.value.trim()) {
        todoStore.addTodoList(title.value)
        title.value = ''
    } else {
        showWarningDialog('事项组名称不能为空')
    }
}
function addTodo(index: number) {
    if (todo.value.trim()) {
        todoStore.addTodo(todo.value, index)
        todo.value = ''
    } else {
        showWarningDialog('待办事项不能为空')
    }
}

function stateChange(item: Todo, index: number, subindex: number) {
    item.completed =! item.completed
    todoStore.updateTodo(item, index, subindex)
}
</script>

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
    user-select: none;
}
.todo>.options {
    display: flex;
}
.todo>.options>div,.todo>.groups>li>.title>div {
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
.todo>.options>:last-child {
    color: rgb(152,195,121);
}
.todo>.options>div:active,.todo>.groups>li>.title>div:active {
    background-color: rgba(180,180,180,0.25);
}
.todo>.options>input {
    font-size: 1.5rem;
    flex: 1;
    padding: 0 0.5rem;
    background-color: rgb(40,44,52);
    outline: none;
    transition: all 0.1s;
    border: none;
}
.todo>.groups {
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: scroll;
    background-color: rgb(33,37,43);
    list-style: none;
    box-sizing: border-box;
    border: 0.5rem solid rgb(40,44,52);
}
.todo>.groups>li {
    padding: 0.5rem 1rem;
    margin: 1rem 0;
    width: 80%;
}
.todo>.groups>li>.title {
    display: flex;
}
.todo>.groups>li>.title>div {
    color: rgb(224,108,117);
    opacity: 0;
}
.todo>.groups>li>.title:hover>div {
    opacity: 1;
}
.todo>.groups>li>span {
    font-size: 1.5rem;
    line-height: 2rem;
    user-select: none;
    margin: 0 0.5rem;
}
.todo>.groups>li>.add {
    display: flex;
    margin: 0.5rem 0;
}
.todo>.groups>li>.title>p,.todo>.groups>li>.add>div {
    height: 3rem;
    font-size: 2rem;
    line-height: 3rem;
    user-select: none;
    margin: 0 0.5rem;
}
.todo>.groups>li>.add>input {
    font-size: 1.5rem;
    flex: 1;
    border: none;
    outline: none;
    padding: 0 0.5rem;
    background-color: rgb(40,44,52);
}
.todo>.groups>li>.add>div {
    cursor: pointer;
    transition: all 0.1s;
    color: rgb(152,195,121);
}
.todo>.groups>li>.add>div:active {
    transform: scale(1.5);
}
.todo>.groups>li>.items {
    display: flex;
    flex-direction: column;
}
.todo>.groups>li>.items>li {
    display: flex;
}
.todo>.groups>li>.items>li>p,.todo>.groups>li>.items>li>div {
    height: 3rem;
    margin-right: 0.5rem;
    line-height: 3rem;
    user-select: none;
}
.todo>.groups>li>.items>li>p {
    font-size: 1.5rem;
    text-indent: 3em;
    transition: all 0.1s;
}
.todo>.groups>li>.items>li>div {
    font-size: 2rem;
    margin: 0 0.5rem;
    opacity: 0;
    transition: all 0.1s;
    cursor: pointer;
}
.todo>.groups>li>.items>li>div:active {
    transform: scale(1.5);
}
.todo>.groups>li>.items>li:hover>div {
    opacity: 1;
}
.todo>.groups>li>.items>li>:last-child {
    color: rgb(224,108,117);
}
.completed {
    color: rgb(152,195,121);
}
</style>