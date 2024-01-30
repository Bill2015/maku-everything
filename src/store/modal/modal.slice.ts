import { createSlice, PayloadAction } from '@reduxjs/toolkit';

// eslint-disable-next-line no-shadow
export enum ModalName {
    importCategory,
    CreateCategory,
    CreateSubject,
    CreateTag,
    CreateResource,
}

interface ModalReduxProps {
    opened: boolean,
}

export interface ModalState {
    modals: {[key in ModalName]: ModalReduxProps}
}

const initialState: ModalState = {
    modals: {
        [ModalName.importCategory]: { opened: false },
        [ModalName.CreateCategory]: { opened: false },
        [ModalName.CreateSubject]:  { opened: false },
        [ModalName.CreateTag]:      { opened: false },
        [ModalName.CreateResource]: { opened: false },
    },
};

const modalSlice = createSlice({
    name:     'modal',
    initialState,
    reducers: {
        setModalOpenStatus: (state, action: PayloadAction<{ name: ModalName, open: boolean }>) => {
            const { name, open } = action.payload;
            state.modals[name] = { ...state.modals[name], opened: open };
        },
    },
});

export const { setModalOpenStatus } = modalSlice.actions;

export default modalSlice.reducer;
