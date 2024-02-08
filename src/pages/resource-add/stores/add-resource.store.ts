import { CategoryResDto } from '@api/category';
import { ResourceCreateDto, ResourceTagDto } from '@api/resource';
import { createStore } from 'zustand';

export type ResourceCreateTagItem = Pick<ResourceTagDto, 'id' | 'name' | 'subject_name'>;

export type ResourceCreateItem = Omit<ResourceCreateDto, 'tags'> & { tags: ResourceCreateTagItem[] }

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
    updateResourceTag: <A extends 'add' | 'delete'>(index: number, action: A, value: A extends 'add' ? ResourceCreateTagItem : string) => void;
    deleteResource: (index: number) => void;
}

// eslint-disable-next-line arrow-body-style
export const createAddResourceStore = (category: CategoryResDto | null) => {
    const DEFAULT_PROPS: AddResourceState = {
        category:       null,
        resources:      [],
        activeResource: null,
    };
    return createStore<AddResourceState & AddResourceActions>((set) => ({
        ...DEFAULT_PROPS,
        category:          category,
        activeResource:    null,
        resources:         [],
        setActiveResource: (index: number) => set((state) => {
            const data = state.resources.at(index);
            return { activeResource: data ? { data: data, index: index } : null };
        }),
        addResource: (data: ResourceCreateItem | ResourceCreateItem[]) => set((state) => {
            let newList = [...state.resources];
            if (Array.isArray(data)) {
                newList = newList.concat(data);
            }
            else {
                newList.push(data);
            }

            return {
                resources:      newList,
                activeResource: { data: newList.at(-1)!, index: newList.length - 1 },
            };
        }),
        updateResourceTag: <A extends 'add' | 'delete'>(index: number, action: A, value: A extends 'add' ? ResourceCreateTagItem : string) => set((state) => {
            const targetRes = state.resources.at(index);
            if (!targetRes) {
                return {};
            }
            switch (action) {
            case 'add':
                targetRes.tags.push(value as ResourceCreateTagItem);
                break;
            case 'delete': {
                const deleteIndex = targetRes.tags.findIndex((item) => item.id === (value as string));
                if (deleteIndex >= 0) {
                    targetRes.tags.splice(deleteIndex, 1);
                }
                break;
            }
            default:
                break;
            }
            if (state.activeResource?.index === index) {
                return {
                    resources:      [...state.resources],
                    activeResource: { data: targetRes, index },
                };
            }
            return { resources: [...state.resources] };
        }),
        updateResource: (index: number, newData: ResourceCreateItem) => set((state) => {
            const newList = [...state.resources];
            newList.splice(index, 1, newData);
            // if active resource equal update index, the active resource should update too
            if (state.activeResource?.index === index) {
                return {
                    resources:      newList,
                    activeResource: { data: newList.at(index)!, index },
                };
            }
            return { resources: newList };
        }),
        deleteResource: (index: number) => set((state) => {
            const newList = [...state.resources];
            newList.splice(index, 1);

            return {
                resources:      newList,
                activeResource: newList.at(index - 1) ? { data: newList.at(index - 1)!, index: (index - 1) } : null,
            };
        }),
    }));
};

export type AddResourceStore = ReturnType<typeof createAddResourceStore>;
