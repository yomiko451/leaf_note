interface Note {
    id: number,
    title: string,
    content: string,
    created_at: string,
    updated_at: string,
    tags: Array<string>,
    starred: boolean
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

interface Config {
    cover_filter: boolean,
    cover_url: string,
    font_size: number,
    city: string,
    weather: Weather
}

export type {
    Note,
    TodoList,
    Todo,
    Weather,
    Config
}