import {defineStore} from 'pinia'
import {ref} from 'vue'
import {TodoList, Todo} from '../types'
import {invoke} from '@tauri-apps/api/tauri'

export const useTodoStore = defineStore('todo', ()=>{
    const todoListArr = ref<Array<TodoList>>([])
    
    
    const loadTodoArr = (givenTodoArr: Array<TodoList>) =>{
        todoListArr.value = givenTodoArr
    }

    const addTodoList = async (title: string) => {
        const newTodoList: TodoList = await invoke('create_todo_list', {title})
        todoListArr.value.unshift(newTodoList)
    } 

    const deleteTodoList = async (index: number)=>{
        const todoList = todoListArr.value[index]
        await invoke('delete_todo_list', {todoList: todoList})
        todoListArr.value.splice(index, 1)
    }

    const saveTodoList = async (item: TodoList)=>{
        await invoke('save_todo_list', {todoList: item})
    }

    const addTodo = async (content: string, index: number)=>{
        const newTodo: Todo = await invoke('create_todo', {content})
        todoListArr.value[index].content.push(newTodo)
        saveTodoList(todoListArr.value[index])
    }

    const deleteTodo = (index: number, subIndex: number)=>{
        todoListArr.value[index].content.splice(subIndex, 1)
        saveTodoList(todoListArr.value[index])
    }

    const updateTodo = (item: Todo, index: number, subIndex: number)=>{
        todoListArr.value[index].content[subIndex] = item
        saveTodoList(todoListArr.value[index])
    }

    return {
        todoListArr,
        loadTodoArr,
        addTodoList,
        deleteTodoList,
        addTodo,
        updateTodo,
        deleteTodo
    }
})