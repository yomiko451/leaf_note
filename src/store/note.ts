import {defineStore} from 'pinia'
import {ref} from 'vue'
import {Note} from '../types'
import { invoke } from '@tauri-apps/api/tauri'
import router from '../router'

export const useNoteStore = defineStore('note', ()=>{
    const notes = ref<Array<Note>>([])
    const emptyNote: Note = {
        id: -1,
        title: '',
        content: '',
        created_at: '',
        updated_at: '',
        tags: [],
        starred: false
    }
    const selectedNote = ref<Note>(emptyNote)
    const selectedNoteIndex = ref<number>(-1)

    const updateNotes = (givenNotes: Array<Note>) => {
        notes.value = givenNotes
    }

    const addNote = async () => {
        const note: Note = await invoke('create_note')
        selectedNoteIndex.value = 0
        selectedNote.value = note
        notes.value.unshift(note)
    }

    const saveNote = async () => {
        const time: string = await invoke('get_time');
        selectedNote.value.updated_at = time
        notes.value[selectedNoteIndex.value] = selectedNote.value
        await invoke('save_note', {note: selectedNote.value})
    }

    const updateSelectedNote = (note: Note, index: number) => {
        const {id, title, content, created_at, updated_at, tags, starred} = note
        const newNote: Note = {id, title, content, created_at, updated_at, tags, starred}
        selectedNote.value = newNote
        selectedNoteIndex.value = index
    }

    const changeNoteStarred = () => {
        selectedNote.value.starred = !selectedNote.value.starred
    }

    const deleteNote = async (item: Note, index: number) => {
        await invoke('delete_note', {item})
        notes.value.splice(index, 1)
        reset()
    }

    const addTag = (tag: string) => {
        selectedNote.value.tags.push(tag)
    }

    const deleteTag = (index: number) => {
        selectedNote.value.tags.splice(index, 1);
    }

    const reset = ()=>{
        if (notes.value.length > 0) {
            selectedNote.value = notes.value[0],
            selectedNoteIndex.value = 0
            router.push('/content')
        } else {
            selectedNote.value = emptyNote,
            selectedNoteIndex.value = -1,
            router.push({
                path: '/empty',
                query: {
                    text: '单击左侧按钮添加新笔记'
                }
            })
        }
    }

    return {
        notes,
        selectedNote,
        selectedNoteIndex,
        updateNotes,
        addNote,
        saveNote,
        changeNoteStarred,
        updateSelectedNote,
        deleteNote,
        addTag,
        deleteTag,
        reset
    }
})