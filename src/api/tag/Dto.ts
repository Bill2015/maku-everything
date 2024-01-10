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

    category_name: string;

    belong_subject: string;

    subject: string;

    tagged_count: number;

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
