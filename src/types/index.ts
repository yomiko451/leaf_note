interface Note {
    id: number,
    title: string,
    content: string,
    created_at: string,
    updated_at: string,
    tags: Array<string>,
    stared: boolean,
}

interface TodoList {
    id: number,
    title: string,
    content: Array<Todo>,
    created_at: string
}

interface Todo {
    id: number,
    content: string,
    completed: boolean
}

export type {
    Note,
    TodoList,
    Todo
}