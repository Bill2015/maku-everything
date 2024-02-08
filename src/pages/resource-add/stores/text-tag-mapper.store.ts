import { createStore } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { enableMapSet } from 'immer';

enableMapSet();

export type TextTagValue = {
    id: string,
    name: string,
    subjectName: string,
};

export type TextTagValueItem = {
    key: string,
    indexId: number,
    value: TextTagValue | null
};

type TextTagValueWithoutKeyText = Omit<TextTagValue, 'keyText'>;

type TextTagMapperState = {
    highlightText: string;
    textMap: Map<string, TextTagValueItem>;
}

type TextTagMapperActions = {
    setHighlightText: (val: string) => void;
    textMapInsert: (key: string, val: TextTagValueWithoutKeyText | null) => void;
    textMapDelete: (key: string) => void;
}

// eslint-disable-next-line arrow-body-style
export const createTextTagMapperStore = (defaultTextMap: Record<string, TextTagValueWithoutKeyText>) => {
    // for add order sorting
    let indexId = 0;
    const map = new Map<string, TextTagValueItem>();
    Object.entries(defaultTextMap)
        .forEach(([key, value]) => {
            indexId += 1;
            map.set(key, {
                indexId,
                value,
                key,
            });
        });
    return createStore<TextTagMapperState & TextTagMapperActions>()(
        immer((set) => ({
            highlightText:    '',
            textMap:          map,
            setHighlightText: (value: string) => set((state) => {
                state.highlightText = value;
            }),
            textMapInsert: (key: string, value: TextTagValueWithoutKeyText | null) => set(({ textMap }) => {
                const newValue = value ? { keyText: key, ...value } : null;
                if (textMap.has(key)) {
                    // eslint-disable-next-line no-param-reassign
                    textMap.get(key)!.value = newValue;
                }
                indexId += 1;
                textMap.set(key, {
                    value: newValue,
                    key,
                    indexId,
                });
            }),
            textMapDelete: (key: string) => set((state) => {
                state.textMap.delete(key);
                if (state.highlightText === key) {
                    state.highlightText = '';
                }
            }),
        })),
    );
};

export type TextTagMapperStore = ReturnType<typeof createTextTagMapperStore>;
