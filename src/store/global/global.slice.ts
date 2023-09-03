import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface ActiveCategory {
    id: string,
    name: string,
}

export interface GlobalState {
    activeCategory: ActiveCategory | null;
}

const initialState: GlobalState = { activeCategory: null };

const globalSlice = createSlice({
    name:     'global',
    initialState,
    reducers: {
        setActiveCategory: (state, action: PayloadAction<ActiveCategory>) => {
            state.activeCategory = action.payload;
        },
    },
});

export const { setActiveCategory } = globalSlice.actions;

export default globalSlice.reducer;
