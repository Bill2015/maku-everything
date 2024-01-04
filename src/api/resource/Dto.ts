export interface ResourceFileDto {
    uuid: string,

    name: string,

    path: string,

    root: string,

    ext: string,
}

export interface ResourceResDto {
    id: string,

    name: string,

    description: string,

    file: ResourceFileDto,

    created_at: string,

    updated_at: string,
}

export interface ResourceCreateDto {
    name: string,

    description: string,

    belong_category: string,

    file_path: string,

    url_path: string,
}

export interface ResourceTagOperateDto {
    id: string;

    tag_id: string;
}

export interface ResourceTagDto {
    id: string,

    name: string,

    description: string,

    belong_subject: string,

    subject_name: string,

    created_at: string,

    updated_at: string,
}

export interface ResourceDetailDto {
    id: string,

    name: string,

    description: string,

    file: ResourceFileDto,

    created_at: string,

    updated_at: string,

    tags: ResourceTagDto[],
}

export interface QueryResoruceDto {
    id?: string;

    name?: string;

    belong_category?: string;

    order_by?: string;
}
