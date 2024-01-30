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
    isConfirm: boolean,
    isCancel: boolean,
}

export interface ModalState {
    modals: {[key in ModalName]: ModalReduxProps}
}

const initialState: ModalState = {
    modals: {
        [ModalName.importCategory]: {
            opened:    false,
            isConfirm: false,
            isCancel:  false,
        },
        [ModalName.CreateCategory]: {
            opened:    false,
            isConfirm: false,
            isCancel:  false,
        },
        [ModalName.CreateSubject]: {
            opened:    false,
            isConfirm: false,
            isCancel:  false,
        },
        [ModalName.CreateTag]: {
            opened:    false,
            isConfirm: false,
            isCancel:  false,
        },
        [ModalName.CreateResource]: {
            opened:    false,
            isConfirm: false,
            isCancel:  false,
        },
    },
};

const modalSlice = createSlice({
    name:     'modal',
    initialState,
    reducers: {
        setModalOpenStatus: (state, action: PayloadAction<{ name: ModalName, status: Partial<ModalReduxProps> }>) => {
            const { name, status } = action.payload;
            state.modals[name] = { ...state.modals[name], ...status };
        },
    },
});

export const { setModalOpenStatus } = modalSlice.actions;

export default modalSlice.reducer;
