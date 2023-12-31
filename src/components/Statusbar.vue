<template>
    <div class="statusbar">
        <p>99/99</p>
        <p>{{ config?.city }}</p>
        <p>{{ config?.weather.date }}</p>
        <p>{{ config?.weather.weather }}</p>
        <p>{{ config?.weather.temperature }}</p>
        <p>{{ config?.weather.direct }}</p>
        <p>00000000.png</p>
    </div>
</template>

<script lang="ts" setup>
import { useConfigStore } from '../store/config';
import { storeToRefs } from 'pinia';

const configStore = useConfigStore();
const {config} = storeToRefs(configStore);

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