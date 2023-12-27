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
        selectedNoteIndex.value = 0
        selectedNote.value = note
        notes.value.unshift(note)
    }

    const saveNote = async () => {
        if (typeof selectedNoteIndex.value === 'number') {
            await invoke('save_note', {note: notes.value[selectedNoteIndex.value]})
        }
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
        saveNote,
        updateNote,
        deleteNote
    }
})