<template>
    <div class="statusbar">
        <p>99/99</p>
        <p>{{ weather.date }}</p>
        <p>{{ weather.weather }}</p>
        <p>{{ weather.temperature }}</p>
        <p>{{ weather.direct }}</p>
        <p>00000000.png</p>
    </div>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api';
import { Weather } from '../types';
import { ref } from 'vue';

const weather = ref<Weather>({
    date: '',
    temperature: '',
    weather: '',
    direct: ''
})

const getWeather = async ()=> {
    weather.value = await invoke('get_weather');
}

getWeather()

</script>

<style scoped>
.statusbar {
    background-color: rgb(40,44,52);
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 1rem;
    user-select: none;
}
.statusbar>p {
    font-size: 1.5rem;
}
</style>