import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface ModelReduxProps {
    opened: boolean,
}

export interface ModalState {
    createSubject: ModelReduxProps;
}

const initialState: ModalState = { createSubject: { opened: false } };

const modalSlice = createSlice({
    name:     'modal',
    initialState,
    reducers: {
        setCreateSubjectModelOpen: (state, action: PayloadAction<boolean>) => {
            state.createSubject.opened = action.payload;
        },
    },
});

export const { setCreateSubjectModelOpen } = modalSlice.actions;

export default modalSlice.reducer;
