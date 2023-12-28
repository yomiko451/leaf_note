import {defineStore} from 'pinia'
import {ref} from 'vue'
import {TodoList, Todo} from '../types'
import {invoke} from '@tauri-apps/api/tauri'

export const useTodoStore = defineStore('todo', ()=>{
    const todoListArr = ref<Array<TodoList>>([])
    
    
    const addTodoList = async (title: string) => {
        const newTodoList: TodoList = await invoke('create_todo_list', {title})
        todoListArr.value.unshift(newTodoList)
    } 

    const addTodo = async (content: string, index: number)=>{
        const newTodo: Todo = await invoke('create_todo', {content})
        todoListArr.value[index].content.push(newTodo)
    }

    const updateTodo = (item: Todo, index: number, subIndex: number)=>{
        todoListArr.value[index].content[subIndex] = item
    }

    return {
        todoListArr,
        addTodoList,
        addTodo,
        updateTodo
    }
})