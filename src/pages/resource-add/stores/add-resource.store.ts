import { createStore } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { enableMapSet } from 'immer';
import { ResourceCreateDto, ResourceTagDto } from '@api/resource';
import { CategoryResDto } from '@api/category';

enableMapSet();

export type ResourceCreateTagItem = Pick<ResourceTagDto, 'id' | 'name' | 'subject_name'>;

export type ResourceCreateItem = Omit<ResourceCreateDto, 'tags'> & {
    tags: ResourceCreateTagItem[];

    ignoreText: Set<string>;
}

export type ActiveResourceType = { data: ResourceCreateItem, index: number } | null;

type AddResourceState = {
    category: CategoryResDto | null,

    resources: ResourceCreateItem[],

    activeResource: ActiveResourceType,
}

type AddResourceActions = {
    setActiveResource: (index: number) => void;
    addResource: (data: ResourceCreateItem | ResourceCreateItem[]) => void;
    updateResource: (index: number, newData: ResourceCreateItem) => void;
    updateResourceTag: <A extends 'add' | 'delete'>(
        index: number,
        action: A,
        value: A extends 'add' ? ResourceCreateTagItem : string
    ) => void;
    updateResourceIgnoreText: (
        index: number,
        action: 'add' | 'delete',
        text: string,
    ) => void;
    deleteResource: (index: number) => void;
}

// eslint-disable-next-line arrow-body-style
export const createAddResourceStore = (category: CategoryResDto | null) => {
    const DEFAULT_PROPS: AddResourceState = {
        category:       null,
        resources:      [],
        activeResource: null,
    };
    return createStore<AddResourceState & AddResourceActions>()(
        immer((set) => ({
            ...DEFAULT_PROPS,
            category:          category,
            setActiveResource: (index: number) => set((state) => {
                const data = state.resources.at(index);
                state.activeResource = data ? { data: data, index: index } : null;
            }),
            addResource: (data: ResourceCreateItem | ResourceCreateItem[]) => set((state) => {
                state.resources = state.resources.concat(data);
                const len = state.resources.length;
                state.activeResource = (len > 0) ? { index: len - 1, data: state.resources.at(-1)! } : null;
            }),
            updateResourceTag: <A extends 'add' | 'delete'>(
                index: number,
                action: A,
                value: A extends 'add' ? ResourceCreateTagItem : string,
            ) => set((state) => {
                const resource = state.resources.at(index);
                if (!resource) {
                    return;
                }
                switch (action) {
                case 'add':
                    resource!.tags.push(value as ResourceCreateTagItem);
                    break;
                case 'delete': {
                    resource.tags = resource!.tags.filter((item) => item.id !== (value as string));
                    break;
                }
                default:
                    break;
                }
                if (state.activeResource?.index === index) {
                    state.activeResource = { index, data: state.resources.at(index)! };
                }
            }),
            updateResource: (index: number, newData: ResourceCreateItem) => set((state) => {
                state.resources.splice(index, 1, newData);
                // if active resource equal update index, the active resource should update too
                if (state.activeResource?.index === index) {
                    state.activeResource = { index, data: state.resources.at(index)! };
                }
            }),
            deleteResource: (index: number) => set((state) => {
                state.resources.splice(index, 1);
                if (state.activeResource?.index === index) {
                    const prev = state.resources.at(index - 1);
                    state.activeResource = prev ? { index, data: prev } : null;
                }
            }),
            updateResourceIgnoreText: (index: number, action: 'add' | 'delete', text: string) => set((state) => {
                const resource = state.resources.at(index);
                if (!resource) {
                    return;
                }
                switch (action) {
                case 'add':
                    resource.ignoreText.add(text);
                    break;
                case 'delete':
                    resource.ignoreText.delete(text);
                    break;
                default:
                    break;
                }
                if (state.activeResource?.index === index) {
                    state.activeResource = { index, data: state.resources.at(index)! };
                }
            }),
        })),
    );
};

export type AddResourceStore = ReturnType<typeof createAddResourceStore>;
