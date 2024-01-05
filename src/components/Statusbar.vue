<template>
    <div class="statusbar">
        <p>日期：{{ config?.weather.date }}</p>
        <p>地区：{{ config?.city }}</p>
        <p>天气：{{ config?.weather.weather }}</p>
        <p>气温：{{ config?.weather.temperature }}</p>
        <p>风向：{{ config?.weather.direct }}</p>
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
    background-color: var(--primiary-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 1rem;
    user-select: none;
}
.statusbar>p {
    line-height: 2rem;
    font-size: 1.5rem;
}
</style>