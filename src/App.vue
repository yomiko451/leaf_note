<template>
  <div class="container">
    <Notelist/>
    <div class="view">
      <RouterView></RouterView>
    </div>
    <Statusbar/>
    <Contextmenu/>
  </div>
</template>

<script lang="ts" setup>
import Contextmenu from './components/Contextmenu.vue';
import Notelist from './components/Notelist.vue'
import Statusbar from './components/Statusbar.vue'
import { RouterView } from 'vue-router';
import { invoke } from '@tauri-apps/api';
import {Note, TodoList} from './types'
import { useNoteStore } from './store/note'
import { useTodoStore } from './store/todo'
import { useConfigStore } from './store/config';

initialize();

async function initialize() {
  await invoke('check_requirements');
  const notes: Array<Note> = await invoke('load_note');
  const todoListArr: Array<TodoList> = await invoke('load_todo_list');
  notes.reverse()
  todoListArr.reverse()
  useNoteStore().loadNotes(notes);
  useTodoStore().loadTodoArr(todoListArr);
  await useConfigStore().loadConfig()
  useConfigStore().setBasicConfig()
  await invoke('show_main_window');
}
</script>

<style scoped>
.container {
  height: 100vh;
  width: 100vw;
  display: grid;
  grid-template-rows: 1fr 3rem;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
  box-sizing: border-box;
  padding: 0.5rem;
  background-color: var(--secondry-color);
  border-radius: 0;
  position: relative;
}
.notelist {
  grid-row: 1/2;
  grid-column: 1/2;
}
.view {
  grid-row: 1/2;
  grid-column: 2/3;
  min-height: 0;
}
.statusbar {
  grid-row: 2/3;
  grid-column: 1/3;
}
</style>