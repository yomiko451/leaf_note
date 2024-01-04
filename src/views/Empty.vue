<template>
    <div class="empty">
        <p>暂无内容</p>
    </div>
</template>

<script lang="ts" setup>

</script>

<style scoped>
.empty {
    width: 100%;
    height: 100%;
    background-color: var(--primiary-color);
    display: flex;
    align-items: center;
    justify-content: center;
}
.empty>p {
    font-size: 2rem;
    color: var(--letter-color);
    user-select: none;
}
</style>
