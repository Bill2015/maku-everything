import { useGlobalDispatch, useGlobalSelector } from "../hook";
import { ActiveCategory, setActiveCategory as setActiveCategoryRedux } from "./global.slice";

export function useActiveCategoryRedux() {
    const { activeCategory } = useGlobalSelector();
    const dispatch = useGlobalDispatch();

    const setActiveCategory = (category: ActiveCategory) => {
        dispatch(setActiveCategoryRedux(category))
    }

    return { activeCategory, setActiveCategory };
}
