<template>
    <div class="setting">
        <div>
            <p>所在城市</p>
            <div>正常</div>
        </div>
        <div>
            <p>封面滤镜</p>
            <div>安庆</div>
        </div>
        <div>
            <p>封面位置</p>
            <div>正常</div>
        </div>
        <div>
            <p>窗口大小</p>
            <div>正常</div>
        </div>
        <div>
            <p>显示字体</p>
            <div>正常</div>
        </div>
        <div>
            <p>显示字号</p>
            <div>正常</div>
        </div>
    </div>
</template>

<style scoped>
.setting {
    width: 100%;
    height: 100%;
    background-color: rgb(40,44,52);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    user-select: none;
}
.setting>div {
    width: 30rem;
    height: 6rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.setting>div>p {
    font-size: 2rem;
    line-height: 4rem;
}
.setting>div>div {
    box-sizing: border-box;
    line-height: 4rem;
    width: 8rem;
    text-align: center;
    cursor: pointer;
}
.setting>div>div:hover {
    background-color: rgb(180,180,180);
    color: rgb(40,44,52);
}
</style>