<template>
    <div class="todo">
        <h1>{{ CountSum }}</h1>
        <div class="options">
            <input @keyup.enter="addNoteList" v-model="title" type="text" placeholder="请输入事项组名称">
            <div @click="addNoteList">添加</div>
        </div>
        <ol class="groups">
            <li v-for="todoList, index in todoListArr" :key="todoList.id">
                <div class="title">
                    <p>{{ todoList.title }}</p>
                    <p>{{ calculateCount(todoList.content) }}</p>
                    <div @click="deleteTodoList(index, todoList.title)">删除</div>
                </div>
                <span>{{ todoList.created_at }}</span>
                <div class="add">
                    <input @keyup.enter="addTodoKeyboard(index, $event)" type="text" placeholder="请输入待办事项">
                    <div @click="addTodoMouse(index, $event)">✚</div>
                </div>
                <ol class="items">
                    <li v-for="item, subindex in todoList.content" :key="item.id">
                        <p :class="item.completed? 'completed' : ''">{{ (subindex + 1) + ' ' + item.content }}</p>
                        <div @click="stateChange(item, index, subindex)" :class="item.completed? '' : 'completed'">✔</div>
                        <div @click="todoStore.deleteTodo(index, subindex)">✖</div>
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
import {ref, computed} from 'vue'

const todoStore = useTodoStore()
const {todoListArr} = storeToRefs(todoStore)
const {showWarningDialog, showAskDialog} = useDialog()
const title = ref<string>('')

const CountSum = computed(()=>{
    let sum = 0
    let done = 0
    for (let i = 0; i < todoListArr.value.length; i++) {
        done += todoListArr.value[i].content.filter(j => j.completed).length
    }
    for (let i = 0; i < todoListArr.value.length; i++) {
        sum += todoListArr.value[i].content.length
    }
    return `事项总计 ${sum} 已完成 ${done}`
})

function addNoteList() {
    if (title.value.trim()) {
        todoStore.addTodoList(title.value)
        title.value = ''
    } else {
        showWarningDialog('事项组名称不能为空')
    }
}

async function deleteTodoList(index: number, title: string) {
    const res = await showAskDialog(`确定删除 ${title} 事项组吗？`)
    if (res) {
        todoStore.deleteTodoList(index)
    }
}

function addTodoKeyboard(index: number, event: Event) {
    const element = event.target as HTMLInputElement
    addTodo(element, index)
}

function addTodoMouse(index: number, event: Event) {
    const element = event.target as HTMLInputElement
    const bro_element = element.previousElementSibling as HTMLInputElement
    addTodo(bro_element, index)
}

function addTodo(element: HTMLInputElement, index: number) {
    if (element.value.trim()) {
        todoStore.addTodo(element.value.trim(), index)
        element.value = ''
    } else {
        showWarningDialog('待办事项不能为空')
    }
}

function stateChange(item: Todo, index: number, subindex: number) {
    item.completed =! item.completed
    todoStore.updateTodo(item, index, subindex)
}

function calculateCount(content: Array<Todo>) {
    return content.filter(i => i.completed).length + ' / ' + content.length 
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
    background-color: var(--primiary-color);
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
    background-color: var(--primiary-color);
}
.todo>.options>:last-child {
    color: var(--confirm-color);
}
.todo>.options>div:active,.todo>.groups>li>.title>div:active {
    background-color: var(--click-color);
}
.todo>.options>input {
    font-size: 1.5rem;
    flex: 1;
    padding: 0 0.5rem;
    background-color: var(--primiary-color);
    outline: none;
    transition: all 0.1s;
    border: none;
}
.todo>.groups {
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: scroll;
    background-color: var(--primiary-color);
    list-style: none;
}
.todo>.groups>li {
    padding: 0.5rem 1rem;
    margin: 1rem 0;
    width: 90%;
    background-color: var(--click-color);
}
.todo>.groups>li>.title {
    display: flex;
}
.todo>.groups>li>.title>:first-child {
    white-space: nowrap; 
    overflow: hidden; 
    text-overflow: ellipsis;
}
.todo>.groups>li>.title>:nth-child(2) {
    white-space: nowrap; 
}
.todo>.groups>li>.title>div {
    color: var(--warning-color);
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
    background-color: var(--primiary-color);
}
.todo>.groups>li>.add>div {
    cursor: pointer;
    transition: all 0.1s;
    color: var(--confirm-color);
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
    white-space: nowrap; 
    overflow: hidden; 
    text-overflow: ellipsis;
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
    color: var(--warning-color);
}
.completed {
    color: var(--confirm-color);
}
</style>