import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface ModelReduxProps {
    opened: boolean,
}

export interface ModalState {
    createSubject: ModelReduxProps;
    createTag: ModelReduxProps;
}

const initialState: ModalState = {
    createSubject: { opened: false },
    createTag:     { opened: false },
};

const modalSlice = createSlice({
    name:     'modal',
    initialState,
    reducers: {
        setCreateSubjectModelOpen: (state, action: PayloadAction<boolean>) => {
            state.createSubject.opened = action.payload;
        },
        setCreateTagModelOpen(state, action: PayloadAction<boolean>) {
            state.createTag.opened = action.payload;
        },
    },
});

export const {
    setCreateSubjectModelOpen,
    setCreateTagModelOpen,
} = modalSlice.actions;

export default modalSlice.reducer;
