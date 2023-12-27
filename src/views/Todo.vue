<template>
    <div class="todo">
        <h1>待办事项 10 已完成 5</h1>
        <div class="options">
            <input type="text" placeholder="请输入事项组名称">
            <div>添加</div>
            <div>删除</div>
        </div>
        <ol class="groups">
            <li>
                <div class="add">
                    <p>事项组1</p>
                    <input type="text" placeholder="请输入待办事项">
                    <div>✚</div>
                </div>
                <ol class="items">
                    <li>
                        <p>待办事项</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项2333</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项2333</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项2333</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项2333</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                </ol>
            </li>
            <li>
                <div class="add">
                    <p>事项组1</p>
                    <input type="text" placeholder="请输入待办事项">
                    <div>✚</div>
                </div>
                <ol class="items">
                    <li>
                        <p>待办事项</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项2333</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项2333</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项2333</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                    <li>
                        <p>待办事项2333</p>
                        <div>✔</div>
                        <div>✖</div>
                    </li>
                </ol>
            </li>
        </ol>
    </div>
</template>

<style scoped>
.todo {
    display: grid;
    height: 100%;
    width: 100%;
    grid-template-rows: 6rem 3rem 1fr;
    gap: 0.5rem;
}
.todo>h1 {
    height: 6rem;
    line-height: 6rem;
    font-size: 2rem;
    text-align: center;
    background-color: rgb(40,44,52);
    font-weight: normal;
    user-select: none;
}
.todo>.options {
    display: flex;
}
.todo>.options>div {
    height: 3rem;
    width: 6rem;
    font-size: 1.5rem;
    line-height: 3rem;
    text-align: center;
    user-select: none;
    transition: all 0.1s;
    cursor: pointer;
    margin-left: 0.5rem;
    background-color: rgb(40,44,52);
}
.todo>.options>:nth-child(2) {
    color: rgb(152,195,121);
}
.todo>.options>:last-child {
    color: rgb(224,108,117);
}
.todo>.options>div:active {
    background-color: rgba(180,180,180,0.25);
}
.todo>.options>input {
    line-height: 2rem;
    font-size: 1.5rem;
    flex: 1;
    padding: 0 0.5rem;
    background-color: rgb(40,44,52);
    outline: none;
    transition: all 0.1s;
    border: none;
}
.todo>.groups {
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: scroll;
    background-color: rgb(33,37,43);
    list-style: none;
    box-sizing: border-box;
    border: 0.5rem solid rgb(40,44,52);
}
.todo>.groups>li {
    padding: 0.5rem 1rem;
    margin: 1rem 0;
    width: 80%;
}
.todo>.groups>li>.add {
    display: flex;
}
.todo>.groups>li>.add>p,.todo>.groups>li>.add>div {
    height: 3rem;
    font-size: 2rem;
    line-height: 3rem;
    user-select: none;
}
.todo>.groups>li>.add>input {
    line-height: 2rem;
    font-size: 1.5rem;
    flex: 1;
    min-width: 30%;
    border: none;
    outline: none;
    padding: 0 0.5rem;
    background-color: rgb(40,44,52);
    margin: 0 0.5rem;
}
.todo>.groups>li>.add>div {
    cursor: pointer;
    transition: all 0.1s;
    color: rgb(152,195,121);
}
.todo>.groups>li>.add>div:active {
    transform: translateY(-0.5rem);
}
.todo>.groups>li>.items {
    display: flex;
    flex-direction: column;
}
.todo>.groups>li>.items>li {
    display: flex;
}
.todo>.groups>li>.items>li>p,.todo>.groups>li>.items>li>div {
    height: 3rem;
    margin-right: 0.5rem;
    line-height: 3rem;
    user-select: none;
}
.todo>.groups>li>.items>li>p {
    font-size: 1.5rem;
    text-indent: 3em;
}
.todo>.groups>li>.items>li>div {
    font-size: 2rem;
    margin: 0 0.5rem;
    opacity: 0;
    transition: all 0.1s;
    cursor: pointer;
}
.todo>.groups>li>.items>li>div:active {
    transform: translateY(-0.5rem);
}
.todo>.groups>li>.items>li:hover>div {
    opacity: 1;
}
.todo>.groups>li>.items>li>:nth-child(2) {
    color: rgb(152,195,121);
}
.todo>.groups>li>.items>li>:last-child {
    color: rgb(224,108,117);
}
</style>

<script lang="ts" setup>

</script>