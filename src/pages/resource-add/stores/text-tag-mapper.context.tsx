import { PropsWithChildren, createContext, useCallback, useContext, useMemo, useRef } from 'react';
import { useStore } from 'zustand';
import { CategoryMapperRuleItemResDto, CategoryMutation, CategoryResDto } from '@api/category';
import { TextTagMapperStore, TextTagValue, createTextTagMapperStore } from './text-tag-mapper.store';
import { ResourceCreateItem } from './add-resource.store';

const TextTagMapperContext = createContext<TextTagMapperStore | null>(null);

export function useTextTagMapperContext() {
    const store = useContext(TextTagMapperContext);
    if (!store) throw new Error('Missing TextTagMapperContext.Provider in the tree');

    const { textMap: oldTextMap, category, resetModified, ...states } = useStore(store, (state) => state);
    const updateMapper = CategoryMutation.useUpdateRule();

    const checkTextExist = (text: string) => oldTextMap.has(text);

    const textMapperList = useMemo(
        () => Array.from(oldTextMap.entries())
            .sort((a, b) => a[1].indexId - b[1].indexId)
            .map((val) => val[1]),
        [oldTextMap],
    );

    const handleUpdateMapper = useCallback(() => {
        if (!category) {
            return;
        }

        const items = textMapperList
            .filter((val) => val.tagValue)
            .map(({ key, tagValue: value }) => ({ text: key, tag_id: value!.id }));

        updateMapper.mutateAsync({ id: category!.id, rules: items });
        resetModified();
    }, [category, textMapperList, updateMapper, resetModified]);

    const getResourceSpecificTags = useCallback((resource: ResourceCreateItem) => {
        const array: (TextTagValue & { ignored: boolean, text: string })[] = [];

        for (const { key, tagValue } of textMapperList) {
            const text = resource.file_path || resource.url_path;
            if (tagValue && text!.toLowerCase().includes(key.toLowerCase())) {
                const ignored = resource.ignoreText.has(key);
                array.push({
                    ...tagValue,
                    ignored,
                    text: key,
                });
            }
        }
        return array;
    }, [textMapperList]);

    return {
        textMapperList,
        checkTextExist,
        handleUpdateMapper,
        getResourceSpecificTags,
        ...states,
    };
}

export interface TextTagMapperProviderProps extends PropsWithChildren {
    category: CategoryResDto;

    defaultTextMap: CategoryMapperRuleItemResDto[];
}

export function TextTagMapperProvider(props: TextTagMapperProviderProps) {
    const { children, category, defaultTextMap } = props;

    const storeRef = useRef<TextTagMapperStore>();
    if (!storeRef.current) {
        storeRef.current = createTextTagMapperStore(category, defaultTextMap);
    }

    return (
        <TextTagMapperContext.Provider value={storeRef.current}>
            {children}
        </TextTagMapperContext.Provider>
    );
}
