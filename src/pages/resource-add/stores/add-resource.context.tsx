import { PropsWithChildren, createContext, useContext, useRef } from 'react';
import { useStore } from 'zustand';
import { CategoryResDto } from '@api/category';
import { AddResourceStore, createAddResourceStore } from './add-resource.store';

const AddResourceContext = createContext<AddResourceStore | null>(null);

export function useAddResourceContext() {
    const store = useContext(AddResourceContext);
    if (!store) throw new Error('Missing BearContext.Provider in the tree');

    return useStore(store, (state) => state);
}

export function AddResourceProvider(props: PropsWithChildren & { category: CategoryResDto | null }) {
    const { children, category } = props;

    const storeRef = useRef<AddResourceStore>();
    if (!storeRef.current) {
        storeRef.current = createAddResourceStore(category);
    }

    return (
        <AddResourceContext.Provider value={storeRef.current}>
            {children}
        </AddResourceContext.Provider>
    );
}
