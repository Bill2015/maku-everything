export interface ResourceResDto {
    id: string,

    title: string,

    description: string,

    file_id: string,

    file_name: string,

    file_path: string,

    file_type: string,

    created_at: string,

    updated_at: string,
}

export interface ResourceCreateDto {
    title: string,

    description: string,

    belong_category: string,

    file_path: string,
}
