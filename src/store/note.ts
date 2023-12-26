import {defineStore} from 'pinia'
import {ref} from 'vue'
import {Note} from '../types'
import { invoke } from '@tauri-apps/api/tauri'


export const useNoteStore = defineStore('note', ()=>{
    const notes = ref<Array<Note>>([])
    const selectedNote = ref<Note | null>(null)
    const selectedNoteIndex = ref<number | null>(null)

    const addNote = async () => {
        const note: Note = await invoke('create_note')
        selectedNoteIndex.value = notes.value.length - 1
        selectedNote.value = note
        notes.value.push(note)
    }

    const updateNote = (note: Note) => {
        if (typeof selectedNoteIndex.value === 'number') {
            notes.value[selectedNoteIndex.value] = note
        }
    }

    const deleteNote = (index: number) => {
        notes.value.splice(index, 1)
    }

    return {
        notes,
        selectedNote,
        selectedNoteIndex,
        addNote,
        updateNote,
        deleteNote
    }
})