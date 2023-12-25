interface Note {
    id: number,
    title: string,
    content: string,
    created_at: string,
    updated_at: string,
    tags: Array<string>,
    stared: boolean,
    path: string
}

export type {
    Note
}