export interface ResourceFileDto {
    uuid: string,

    name: string,

    path: string,

    ext: string,
}

export interface ResourceResDto {
    id: string,

    title: string,

    description: string,

    file: ResourceFileDto,

    created_at: string,

    updated_at: string,
}

export interface ResourceCreateDto {
    title: string,

    description: string,

    belong_category: string,

    file_path: string,
}
