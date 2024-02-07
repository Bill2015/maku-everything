import { Map as ImmutableMap } from 'immutable';
import { createStore } from 'zustand';

type TextTagValueType = { tagId: string | null, indexId: number };

type TextTagMapperState = {
    highlightText: string;
    textMap: ImmutableMap<string, TextTagValueType>;
}

type TextTagMapperActions = {
    setHighlightText: (val: string) => void;
    textMapInsert: (key: string, val: string | null) => void;
    textMapDelete: (key: string) => void;
}

// eslint-disable-next-line arrow-body-style
export const createTextTagMapperStore = () => {
    // for add order sorting
    let indexId = 0;
    return createStore<TextTagMapperState & TextTagMapperActions>((set) => ({
        highlightText:    '',
        textMap:          ImmutableMap(),
        setHighlightText: (value: string) => set(() => ({ highlightText: value })),
        textMapInsert:    (key: string, val: string | null) => set((state) => {
            if (state.textMap.has(key)) {
                return { textMap: state.textMap.set(key, { ...state.textMap.get(key)!, tagId: val }) };
            }
            indexId += 1;
            return { textMap: state.textMap.set(key, { tagId: val, indexId: indexId }) };
        }),
        textMapDelete: (key: string) => set((state) => ({ textMap: state.textMap.remove(key) })),
    }));
};

export type TextTagMapperStore = ReturnType<typeof createTextTagMapperStore>;
