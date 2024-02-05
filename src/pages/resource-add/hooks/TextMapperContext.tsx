import { Map as ImmutableMap } from 'immutable';
import React, { PropsWithChildren, useContext, useState } from 'react';

type TextTagMapperContextType = {
    highlightText: string;
    setHighlightText: (val: string) => void;
    textMap: ImmutableMap<string, string | null>;
    textMapInsert: (key: string, val: string | null) => void;
    textMapDelete: (key: string) => void;
}

const TextTagMapperContext = React.createContext<TextTagMapperContextType>({
    highlightText:    '',
    setHighlightText: () => {},
    textMap:          ImmutableMap(),
    textMapInsert:    () => {},
    textMapDelete:    () => {},
});

export function useTextTagMapperContext() {
    return useContext(TextTagMapperContext);
}

export function TextTagMapperProvider(props: PropsWithChildren) {
    const { children } = props;

    const setHighlightText = (val: string) => {
        setState((prev) => ({ ...prev, highlightText: val }));
    };

    const textMapInsert = (key: string, val: string | null) => {
        setState((prev) => ({ ...prev, textMap: prev.textMap.set(key, val) }));
    };
    const textMapDelete = (key: string) => {
        setState((prev) => ({ ...prev, textMap: prev.textMap.delete(key) }));
    };

    const [state, setState] = useState<TextTagMapperContextType>({
        highlightText: '',
        textMap:       ImmutableMap(),
        setHighlightText,
        textMapInsert,
        textMapDelete,
    });

    return (
        <TextTagMapperContext.Provider value={state}>
            {children}
        </TextTagMapperContext.Provider>
    );
}
