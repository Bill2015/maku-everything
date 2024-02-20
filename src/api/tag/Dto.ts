// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace TagAttrPayload {
    export type Variant = 'normal' | 'number' | 'text' | 'date' | 'bool';
    export type All =
        TagAttrPayload.Normal |
        TagAttrPayload.Number |
        TagAttrPayload.Text |
        TagAttrPayload.Date |
        TagAttrPayload.Bool;

    // eslint-disable-next-line @typescript-eslint/ban-types
    export type Normal = null;
    export type Number = { start: number, end: number, defval: number };
    export type Text = { defval: string };
    export type Date = { defval: string };
    export type Bool = { defval: boolean };
    export type AsType<T extends Variant> =
        T extends 'normal' ? TagAttrPayload.Normal :
        T extends 'number' ? TagAttrPayload.Number :
        T extends 'text' ? TagAttrPayload.Text :
        T extends 'date' ? TagAttrPayload.Date : TagAttrPayload.Bool;

    export function As<T extends TagAttrPayload.Variant>(attrval: TagAttrPayload.All) {
        return attrval as TagAttrPayload.AsType<T>;
    }

    export const DEFAULT_VALUE: {[T in Variant]: AsType<T>} = {
        normal: null,
        number: {
            start:  0,
            end:    100,
            defval: 50,
        },
        text: { defval: '' },
        date: { defval: '' },
        bool: { defval: false },
    };
}

export type TagAttrValue = {
    tag_type: TagAttrPayload.Variant,

    attr: TagAttrPayload.All,
}

export type TagCreateDto = {

    name: string;

    description: string;

    belong_subject: string;

    belong_category: string;
} & TagAttrValue;

export type TagResDto = {
    id: string;

    name: string;

    description: string;

    belong_category: string;

    category_name: string;

    belong_subject: string;

    subject_name: string;

    tagged_count: number;

    created_at: string;

    updated_at: string;
} & TagAttrValue;

export interface QueryTagDto {
    id?: string;

    name?: string;

    belong_category?: string;

    belong_subject?: string;

    tagging_resource?: string;

    order_by?: string;

    limit?: number;

    start?: number;
}
