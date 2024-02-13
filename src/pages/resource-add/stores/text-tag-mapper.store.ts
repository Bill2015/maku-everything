import { createStore } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { enableMapSet } from 'immer';
import { CategoryAddRuleItemResDto } from '@api/category';

enableMapSet();

export type TextTagValue = CategoryAddRuleItemResDto['tag'];

export type TextTagValueItem = {
    key: string,
    indexId: number,
    value: TextTagValue
};

type TextTagMapperState = {
    modified: boolean;
    highlightText: string;
    textMap: Map<string, TextTagValueItem>;
}

type TextTagMapperActions = {
    setHighlightText: (val: string) => void;
    textMapInsert: (key: string, val: TextTagValue | null) => void;
    textMapDelete: (key: string) => void;
}

// eslint-disable-next-line arrow-body-style
export const createTextTagMapperStore = (defaultTextMap: CategoryAddRuleItemResDto[]) => {
    // for add order sorting
    let indexId = 0;
    const map = new Map<string, TextTagValueItem>();
    defaultTextMap.forEach(({ text, tag }) => {
        indexId += 1;
        map.set(text, {
            indexId: indexId,
            key:     text,
            value:   tag,
        });
    });
    const defaultState: TextTagMapperState = {
        modified:      false,
        highlightText: '',
        textMap:       map,
    };
    return createStore<TextTagMapperState & TextTagMapperActions>()(
        immer((set) => ({
            ...defaultState,
            setHighlightText: (value: string) => set((state) => {
                state.highlightText = value;
            }),
            textMapInsert: (key: string, newValue: TextTagValue | null) => set((state) => {
                if (state.textMap.has(key)) {
                    // eslint-disable-next-line no-param-reassign
                    state.textMap.get(key)!.value = newValue;
                    return;
                }
                indexId += 1;
                state.textMap.set(key, {
                    value: newValue,
                    key,
                    indexId,
                });
                state.modified = true;
            }),
            textMapDelete: (key: string) => set((state) => {
                state.textMap.delete(key);
                if (state.highlightText === key) {
                    state.highlightText = '';
                }
                state.modified = true;
            }),
        })),
    );
};

export type TextTagMapperStore = ReturnType<typeof createTextTagMapperStore>;
