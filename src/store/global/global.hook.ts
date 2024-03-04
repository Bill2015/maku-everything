import { useCallback, useMemo } from 'react';
import { UpdateConfigDto } from '@api/config';
import { useGlobalDispatch, useGlobalSelector } from '../hook';
import GlobalLocalStorage from './global.local';
import { ActiveCategory, fetchConfigThunk, setActiveCategory as setActiveCategoryRedux, updateConfigThunk } from './global.slice';

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

export function useConfigRedux() {
    const { config, configChanged } = useGlobalSelector();
    const dispatch = useGlobalDispatch();

    const reloadConfig = useCallback(() => {
        dispatch(fetchConfigThunk());
    }, [dispatch]);

    const updateConfig = useCallback((data: UpdateConfigDto) => {
        dispatch(updateConfigThunk(data));
    }, [dispatch]);

    return {
        config, reloadConfig, updateConfig, configChanged,
    };
}
