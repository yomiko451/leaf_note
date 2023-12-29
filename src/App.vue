<template>
  <div class="container">
    <Infobar/>
    <Notelist/>
    <div class="view">
      <RouterView></RouterView>
    </div>
    <Statusbar/>
  </div>
</template>

<script lang="ts" setup>
import Infobar from './components/Infobar.vue'
import Notelist from './components/Notelist.vue'
import Statusbar from './components/Statusbar.vue'
import { RouterView } from 'vue-router';
import { invoke } from '@tauri-apps/api';
import {Note, TodoList} from './types'
import { useNoteStore } from './store/note'
import { useTodoStore } from './store/todo'

initialize();

async function initialize() {
  await invoke('initialize');
  const notes: Array<Note> = await invoke('load_note');
  const todoListArr: Array<TodoList> = await invoke('load_todo_list');
  notes.reverse()
  todoListArr.reverse()
  useNoteStore().$patch({notes});
  useTodoStore().$patch({todoListArr});
}
</script>

<style scoped>
.container {
  height: 100vh;
  width: 100vw;
  display: grid;
  grid-template-rows: 6rem 1fr 3rem;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
  box-sizing: border-box;
  padding: 0.5rem;
  background-color: rgb(33,37,43);
  border-radius: 0;
}
.infobar {
  grid-row: 1/2;
  grid-column: 1/3;
}
.notelist {
  grid-row: 2/3;
  grid-column: 1/2;
}
.view {
  grid-row: 2/3;
  grid-column: 2/3;
  min-height: 0;
}
.statusbar {
  grid-row: 3/4;
  grid-column: 1/3;
}
</style>