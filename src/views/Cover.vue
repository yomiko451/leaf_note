<template>
    <div class="cover">
        <div class="cover-img" :style="{backgroundImage: 'url(' + url + ')'}"></div>
    </div>
</template>

<script lang="ts" setup>
import {ref} from 'vue'

const url = ref<string>('/src/assets/test.jpg')
</script>

<style scoped>
.cover {
    width: 100%;
    height: 100%;
}
.cover>.cover-img {
    width: 100%;
    height: 100%;
    background-size: cover;
    filter: grayscale(1);
}

</style>