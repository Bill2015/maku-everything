import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface ModelReduxProps {
    opened: boolean,
}

export interface ModalState {
    importCategory: ModelReduxProps;
    createSubject: ModelReduxProps;
    createTag: ModelReduxProps;
    createResource: ModelReduxProps;
}

const initialState: ModalState = {
    importCategory: { opened: false },
    createSubject:  { opened: false },
    createTag:      { opened: false },
    createResource: { opened: false },
};

const modalSlice = createSlice({
    name:     'modal',
    initialState,
    reducers: {
        setImportCategoryModelOpen: (state, action: PayloadAction<boolean>) => {
            state.importCategory.opened = action.payload;
        },
        setCreateSubjectModelOpen: (state, action: PayloadAction<boolean>) => {
            state.createSubject.opened = action.payload;
        },
        setCreateTagModelOpen(state, action: PayloadAction<boolean>) {
            state.createTag.opened = action.payload;
        },
        setCreateResourceModelOpen(state, action: PayloadAction<boolean>) {
            state.createResource.opened = action.payload;
        },
    },
});

export const {
    setImportCategoryModelOpen,
    setCreateSubjectModelOpen,
    setCreateTagModelOpen,
    setCreateResourceModelOpen,
} = modalSlice.actions;

export default modalSlice.reducer;
