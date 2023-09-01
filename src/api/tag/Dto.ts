export interface TagCreateDto {

    name: string;

    description: string;

    belong_subject: string;

    belong_category: string;
}

export interface TagResDto {
    id: string;

    name: string;

    description: string;

    belong_category: string;

    belong_subject: string;

    created_at: string;

    updated_at: string;
}

export interface QueryTagDto {
    id?: string;

    name?: string;

    belong_category?: string;

    belong_subject?: string;

    tagging_resource?: string;

    order_by?: string;
}
