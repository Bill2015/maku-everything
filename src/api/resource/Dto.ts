import { TagAttrValue } from '@api/tag';

export interface ResourceUrlDto {
    full: string;

    host: string;
}

export interface ResourceFileDto {
    uuid: string,

    name: string,

    path: string,

    ext: string,

    media_type: string,
}

export interface ResourceResDto {
    id: string,

    name: string,

    description: string,

    root_path: string,

    file?: ResourceFileDto,

    url?: ResourceUrlDto,

    created_at: string,

    updated_at: string,
}

export interface ResourceCreateDto {
    name: string,

    description: string,

    belong_category: string,

    tags?: string[] | null,

    file_path?: string | null,

    url_path?: string | null,
}

export interface ResourceRenameFileDto {
    id: string,

    new_name?: string,
}

export interface ResourceUpdateDto {
    id: string,

    name?: string,

    description?: string,

    auth?: boolean,
}

export interface ResourceTagOperateDto {
    id: string;

    tag_id: string;
}

export type ResourceTagAttrValDto = null | number | string | boolean;

export interface ResourceUpdateTagDto {
    id: string;

    tag_id: string;

    attrval: ResourceTagAttrValDto,
}

export type ResourceTagDto = {
    id: string,

    name: string,

    description: string,

    belong_subject: string,

    subject_name: string,

    tagged_count: number,

    created_at: string,

    added_at: string,

    attrval: ResourceTagAttrValDto,

    updated_at: string,
} & TagAttrValue

export interface ResourceDetailDto {
    id: string,

    name: string,

    description: string,

    root_path: string,

    file?: ResourceFileDto,

    url?: ResourceUrlDto,

    created_at: string,

    updated_at: string,

    tags: ResourceTagDto[],
}

export interface QueryResoruceDto {
    id?: string;

    name?: string;

    belong_category?: string;

    order_by?: string;

    limit?: number;

    start?: number;
}
