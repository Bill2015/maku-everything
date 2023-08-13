import { InvokeArgs } from '@tauri-apps/api/tauri';

export interface CategoryResDto {
    id: string,

    title: string,

    resource_num: number,

    description: string,

    auth: boolean,

    created_at: string,

    updated_at: string,
}

export interface CategoryCreateDto {
    title: string,

    description: string;
}

export interface CategoryUpdateDto extends Partial<CategoryCreateDto> {
    id: string;
}
