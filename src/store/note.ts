import {defineStore} from 'pinia'
import {ref} from 'vue'
import {Note} from '../types'
import { invoke } from '@tauri-apps/api/tauri'

export const useNoteStore = defineStore('note', ()=>{
    const notes = ref<Array<Note>>([])
    const emptyNote: Note = {
        id: -1,
        title: '',
        content: '',
        created_at: '',
        updated_at: '',
        tags: [],
        locked: false
    }
    const selectedNote = ref<Note>(emptyNote)

    const loadNotes = (givenNotes: Array<Note>) => {
        notes.value = givenNotes
    }

    const addNote = async () => {
        const note: Note = await invoke('create_note')
        selectedNote.value = note
        notes.value.unshift(note)
    }

    const saveNote = async () => {
        const time: string = await invoke('get_time');
        selectedNote.value.updated_at = time
        for (let i = 0; i < notes.value.length; i++) {
            if (notes.value[i].id === selectedNote.value.id) {
                notes.value[i] = selectedNote.value
                break
            }
        }
        await invoke('save_note', {note: selectedNote.value})
    }

    const updateSelectedNote = (note: Note) => {
        const {id, title, content, created_at, updated_at, tags, locked} = note
        const newNote: Note = {id, title, content, created_at, updated_at, tags, locked}
        selectedNote.value = newNote
    }

    const getNoteIndex = (note: Note) => {
        return notes.value.findIndex(n => n.id === note.id)
    }

    const changeNoteLocked = () => {
        selectedNote.value.locked = !selectedNote.value.locked
    }

    const deleteNote = async (note: Note) => {
        await invoke('delete_note', {note})
        for (let i = 0; i < notes.value.length; i++) {
            if (notes.value[i].id === note.id) {
                notes.value.splice(i, 1)
                break
            }
        }
    }

    const addTag = (tag: string) => {
        selectedNote.value.tags.push(tag)
    }

    const deleteTag = (index: number) => {
        selectedNote.value.tags.splice(index, 1);
    }

    return {
        notes,
        selectedNote,
        loadNotes,
        addNote,
        saveNote,
        changeNoteLocked,
        updateSelectedNote,
        getNoteIndex,
        deleteNote,
        addTag,
        deleteTag
    }
})