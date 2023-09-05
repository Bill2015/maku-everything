export interface CategoryResDto {
    id: string,

    name: string,

    resource_num: number,

    description: string,

    root_path: string;

    auth: boolean,

    created_at: string,

    updated_at: string,
}

export interface CategoryCreateDto {
    name: string,

    description: string;

    root_path: string;
}

export interface CategoryUpdateDto extends Partial<CategoryCreateDto> {
    id: string;
}
