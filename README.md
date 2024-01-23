# leafnote

一个简单、轻量的笔记应用

A simple and lightweight note-taking app

主要是针对我自己的使用场景开发的，功能很简单，主要就笔记（更接近于便笺）和待办两项，此外还有一个图片展示窗口，可以在闲暇之余放几张心仪的图片作为封面观赏。

It is mainly developed for my own use scenario, the function is very simple, mainly notes (closer to sticky notes) and to-do items, in addition, there is a picture display window, you can put a few favorite pictures as the cover to enjoy in your spare time.

天气查询用到了聚合数据的免费api，每天上限50次，如果有更好用的，不限次数的免费天气api欢迎推荐。如果想自己私人使用，可以去聚合数据申请key，然后在spider.rs文件中替换即可。当然，想用其他api也完全没有问题，只是代码需要自己改了。

The weather query uses the free api of aggregated data, with a limit of 50 times per day. If you have a better one, unlimited free weather api is welcome to recommend. If you want to use it privately, you can go to the aggregated data to apply for the key, and then replace it in the spider.rs file. Of course, there is no problem with using other apis, but you need to change the code yourself.

目前还在开发当中，很多东西需要完善，自己并非程序员，写代码只是单纯出于兴趣，所以代码中难免有各种问题，欢迎各位大佬帮忙改正，一同交流讨论！

It is still under development, and many things need to be improved. I am not a programmer, and writing code is purely out of interest, so there are inevitably various problems in the code. Welcome to help correct, exchange and discuss with the experts!

因为各种破事，自己这边时间比较有限，不过还是会尽力维护更新的啦！

Because of various troubles, I have limited time here, but I will try my best to maintain and update it!

本应用基于tauri框架开发，前端使用了vue3。
默认封面图片来自P站，pid:114150535
默认图标来自阿里巴巴矢量图标库，作者:斑头雁-一柒