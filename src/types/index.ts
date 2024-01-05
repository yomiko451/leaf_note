export interface Note {
    id: number,
    title: string,
    content: string,
    created_at: string,
    updated_at: string,
    tags: Array<string>,
    locked: boolean
}

export interface TodoList {
    id: number,
    title: string,
    content: Array<Todo>,
    created_at: string
}

export interface Todo {
    id: number,
    content: string,
    completed: boolean
}

export interface Weather {
    date: string,
    direct: string,
    temperature: string,
    weather: string
}

export interface Config {
    cover_filter: boolean,
    cover_url: string,
    font_size: number,
    font_family: string,
    city: string,
    weather: Weather,
    ui_scale: number,
    ui_style: boolean
}
