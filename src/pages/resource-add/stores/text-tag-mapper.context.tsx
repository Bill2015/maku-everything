import { PropsWithChildren, createContext, useContext, useMemo, useRef } from 'react';
import { useStore } from 'zustand';
import { TextTagMapperStore, createTextTagMapperStore } from './text-tag-mapper.store';

const TextTagMapperContext = createContext<TextTagMapperStore | null>(null);

export function useTextTagMapperContext() {
    const store = useContext(TextTagMapperContext);
    if (!store) throw new Error('Missing TextTagMapperContext.Provider in the tree');
    const { textMap: oldTextMap, ...states } = useStore(store, (state) => state);

    const textMap = useMemo(() => oldTextMap.sortBy((val) => val.indexId).map((val) => val.tagId), [oldTextMap]);

    return { textMap, ...states };
}

export function TextTagMapperProvider(props: PropsWithChildren) {
    const { children } = props;

    const storeRef = useRef<TextTagMapperStore>();
    if (!storeRef.current) {
        storeRef.current = createTextTagMapperStore();
    }

    return (
        <TextTagMapperContext.Provider value={storeRef.current}>
            {children}
        </TextTagMapperContext.Provider>
    );
}
