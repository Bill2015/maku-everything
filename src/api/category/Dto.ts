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

export interface UpdateCategoryAddRuleDto {
    id: string,
    rules: {
        text: string,
        tag_id: string,
    }[]
}

export interface ExportCategoryDto {
    id: string;
}

export interface CategoryImportDto {
    new_root_path: string,

    data: string,
}

export interface CategoryAddRuleItemResDto {
    tag: {
        id: string,

        name: string,

        subject_name: string,
    } | null,

    text: string,
}

export interface CategoryAddRulesResDto {
    id: string,

    name: string,

    root_path: string,

    rules: CategoryAddRuleItemResDto[],
}
