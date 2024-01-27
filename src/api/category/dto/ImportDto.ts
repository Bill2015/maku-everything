export interface CategoryImportDto {
    new_root_path: string,

    category: {
        id: string,
        description: string,
        name: string,
        root_path: string,
        updated_at: string,
        created_at: string,
        auth: boolean,
    },

    subjects: {
        id: string,
        name: string,
        description: string,
        belong_category: string,
        created_at: string,
        updated_at: string,
        auth: boolean,
    }[],

    tags: {
        id: string,
        name: string,
        description: string,
        belong_category: string,
        belong_subject: string,
        created_at: string,
        updated_at: string,
        auth: boolean,
    }[],

    resources: {
        id: string,
        name: string,
        description: string,
        belong_category: string,
        root_path: string,
        file?: string | null,
        url?: string | null,
        created_at: string,
        updated_at: string,
        tags: string[],
        auth: boolean,
    }[],
}
