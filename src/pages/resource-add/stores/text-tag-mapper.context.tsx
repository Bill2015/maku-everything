import { PropsWithChildren, createContext, useContext, useMemo, useRef } from 'react';
import { useStore } from 'zustand';
import { TextTagMapperStore, TextTagValue, createTextTagMapperStore } from './text-tag-mapper.store';

const TextTagMapperContext = createContext<TextTagMapperStore | null>(null);

export function useTextTagMapperContext() {
    const store = useContext(TextTagMapperContext);
    if (!store) throw new Error('Missing TextTagMapperContext.Provider in the tree');
    const { textMap: oldTextMap, ...states } = useStore(store, (state) => state);

    const checkTextExist = (text: string) => oldTextMap.has(text);

    const textMapperList = useMemo(
        () => Array.from(oldTextMap.entries())
            .sort((a, b) => a[1].indexId - b[1].indexId)
            .map((val) => val[1]),
        [oldTextMap],
    );

    return {
        textMapperList, checkTextExist, ...states,
    };
}

export function TextTagMapperProvider(props: PropsWithChildren & { defaultTextMap: Record<string, TextTagValue> }) {
    const { children, defaultTextMap } = props;

    const storeRef = useRef<TextTagMapperStore>();
    if (!storeRef.current) {
        storeRef.current = createTextTagMapperStore(defaultTextMap);
    }

    return (
        <TextTagMapperContext.Provider value={storeRef.current}>
            {children}
        </TextTagMapperContext.Provider>
    );
}
