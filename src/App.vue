<template>
  <div class="container">
    <Toolbar/>
    <Notelist/>
    <div class="view">
      <RouterView></RouterView>
    </div>
    <Statusbar/>
  </div>
</template>

<script lang="ts" setup>
import Toolbar from './components/Toolbar.vue'
import Notelist from './components/Notelist.vue'
import Statusbar from './components/Statusbar.vue'
import { RouterView } from 'vue-router';


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
}
.view {
  grid-row: 2/3;
  grid-column: 2/3;
}
</style>