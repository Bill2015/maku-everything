import { ConfigAPI, ConfigResDto, UpdateConfigDto } from '@api/config';
import { createAsyncThunk, createSlice, PayloadAction } from '@reduxjs/toolkit';

export const fetchConfigThunk = createAsyncThunk('global/config/fetch', async (_none) => {
    const config = await ConfigAPI.get();
    return config;
});

export const updateConfigThunk = createAsyncThunk('global/config/update', async (data: UpdateConfigDto) => {
    const response = await ConfigAPI.update(data);
    return response;
});

export interface ActiveCategory {
    id: string,
    name: string,
}

export interface GlobalState {
    activeCategory: ActiveCategory | null;
    config: ConfigResDto | null;
    configChanged: boolean;
}

const initialState: GlobalState = {
    activeCategory: null,
    config:         null,
    configChanged:  false,
};

const globalSlice = createSlice({
    name:     'global',
    initialState,
    reducers: {
        setActiveCategory: (state, action: PayloadAction<ActiveCategory>) => {
            state.activeCategory = action.payload;
        },
    },
    extraReducers(builder) {
        builder.addCase(fetchConfigThunk.fulfilled, (state, action) => {
            state.config = action.payload;
            state.configChanged = false;
        });
        builder.addCase(updateConfigThunk.fulfilled, (state, _action) => {
            state.configChanged = true;
        });
    },
});

export const { setActiveCategory } = globalSlice.actions;

export default globalSlice.reducer;
