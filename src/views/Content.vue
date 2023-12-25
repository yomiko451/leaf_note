<template>
    <div class="content">
        <input type="text" value="dddd">
        <div class="data">
            <span>创建时间：2022-01-01</span>
            <span>修改时间：2023-01-01</span>
            <span>星</span>
        </div>
        <textarea>dddd</textarea>
    </div>
</template>

<style scoped>
.content {
    display: grid;
    height: 100%;
    width: 100%;
    grid-template-rows: 6rem 3rem 1fr;
    gap: 0.5rem;
}
.content>input {
    border: none;
    outline: none;
    font-size: 2rem;
    text-align: center;
    line-height: 6rem;
    background-color: rgb(40,44,52);
}
.content>.data {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 0.5rem;
}
.content>.data>span {
    font-size: 1.5rem;
    text-align: center;
    line-height: 2rem;
}
.content>textarea {
    resize: none;
    outline: none;
    border: none;
    background-color: rgb(40,44,52);
    padding: 0.5rem;
    font-size: 2rem;
    text-align: justify;
    line-height: 3rem;
    box-sizing: border-box;
}
</style>