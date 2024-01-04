import { useMemo } from 'react';
import { useGlobalDispatch, useGlobalSelector } from '../hook';
import GlobalLocalStorage from './global.local';
import { ActiveCategory, setActiveCategory as setActiveCategoryRedux } from './global.slice';

export function useActiveCategoryRedux() {
    const { activeCategory: categoryRedux } = useGlobalSelector();
    const dispatch = useGlobalDispatch();

    const setActiveCategory = (category: ActiveCategory) => {
        dispatch(setActiveCategoryRedux(category));
        GlobalLocalStorage.getInstance().setActiveCategory(category.id, category.name);
    };

    const activeCategory = useMemo(() => categoryRedux || GlobalLocalStorage.getInstance().getActiveCategory(), [categoryRedux]);

    return { activeCategory, setActiveCategory };
}
