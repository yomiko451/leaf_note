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
}
.statusbar {
  grid-row: 3/4;
  grid-column: 1/3;
}
</style>