export interface SubjectResDto {
    id: string,

    name: string,

    description: string,

    auth: boolean,

    created_at: string,

    updated_at: string,
}

export interface SubjectCreateDto {

    name: string,

    description: string,

    belong_category: string,
}

export interface SubjectUpdateDto {

    id: string,

    name?: string,

    description?: string,

    auth?: string,
}
