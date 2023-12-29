import { createRouter, createWebHashHistory } from "vue-router";
import Content from '../views/Content.vue'
import Todo from '../views/Todo.vue'
import Cover from '../views/Cover.vue'
import Empty from '../views/Empty.vue'

const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: '/cover',
            component: Cover
        },
        {   
            path: '/content',
            component: Content
        },
        {
            path: '/todo',
            component: Todo
        },
        {
            path: '/empty',
            component: Empty
        },
        {
            path: '/',
            redirect: '/cover'
        }
    ]
})

export default router