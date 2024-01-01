<template>
    <div class="cover">
        <div :class="config?.cover_filter?'filter':''" :style="cover"></div>
    </div>
</template>

<script lang="ts" setup>
import {computed} from 'vue'
import { useConfigStore } from '../store/config';
import { storeToRefs } from 'pinia';
import { convertFileSrc } from '@tauri-apps/api/tauri'
import defaultImg from '../assets/test.jpg'

const configStore = useConfigStore();
const {config} = storeToRefs(configStore);
const cover = computed(() => {
    if (config.value?.cover_url) {
        const url = convertFileSrc(config.value.cover_url)
        return {
            backgroundImage: `url(${url})`
        }
    } else {
        return {
            backgroundImage: `url(${defaultImg})`
        }   
    }
})
</script>

<style scoped>
.cover {
    width: 100%;
    height: 100%;
}
.cover>div {
    width: 100%;
    height: 100%;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
}
.filter {
    filter: grayscale(1);
}

</style>