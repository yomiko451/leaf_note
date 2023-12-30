interface Note {
    id: number,
    title: string,
    content: string,
    created_at: string,
    updated_at: string,
    tags: Array<string>,
    starred: boolean,
    saved: boolean
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

interface Weather {
    date: string,
    direct: string,
    temperature: string,
    weather: string
}

export type {
    Note,
    TodoList,
    Todo,
    Weather
}