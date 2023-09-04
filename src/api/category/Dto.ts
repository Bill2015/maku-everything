export interface CategoryResDto {
    id: string,

    name: string,

    resource_num: number,

    description: string,

    auth: boolean,

    created_at: string,

    updated_at: string,
}

export interface CategoryCreateDto {
    name: string,

    description: string;
}

export interface CategoryUpdateDto extends Partial<CategoryCreateDto> {
    id: string;
}
