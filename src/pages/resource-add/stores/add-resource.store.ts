import { CategoryResDto } from '@api/category';
import { ResourceCreateDto } from '@api/resource';
import { createStore } from 'zustand';

export type ActiveResourceType = { data: ResourceCreateDto, index: number } | null;

type AddResourceState = {
    category: CategoryResDto | null,
    resources: ResourceCreateDto[],
    activeResource: ActiveResourceType,
}

type AddResourceActions = {
    setActiveResource: (index: number) => void;
    addResource: (data: ResourceCreateDto | ResourceCreateDto[]) => void;
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
        addResource: (data: ResourceCreateDto | ResourceCreateDto[]) => set((state) => {
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
