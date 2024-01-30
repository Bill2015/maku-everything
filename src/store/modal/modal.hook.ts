import { useCallback, useEffect } from 'react';
import { useModalSelector, useModalDispatch } from '../hook';
import {
    ModalName,
    setModalOpenStatus,
} from './modal.slice';

type BaseModalHookFunc = {
    open: () => void,
    confirmClose: () => void,
    cancelClose: () => void,
    close: () => void
}

function useBaseModalHook(name: ModalName): [boolean, BaseModalHookFunc] {
    const { opened } = useModalSelector().modals[name];
    const dispatch = useModalDispatch();

    const open = useCallback(() => {
        dispatch(setModalOpenStatus({
            name:   name,
            status: {
                opened: true, isCancel: false, isConfirm: false,
            },
        }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch]);

    const confirmClose = useCallback(() => {
        dispatch(setModalOpenStatus({
            name:   name,
            status: {
                opened: false, isCancel: false, isConfirm: true,
            },
        }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch]);

    const cancelClose = useCallback(() => {
        dispatch(setModalOpenStatus({
            name:   name,
            status: {
                opened: false, isCancel: true, isConfirm: false,
            },
        }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch]);

    const close = useCallback(() => {
        dispatch(setModalOpenStatus({
            name:   name,
            status: {
                opened: false, isCancel: false, isConfirm: false,
            },
        }));
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch]);

    return [opened, {
        open, confirmClose, cancelClose, close,
    }];
}

export function useImportCategoryModal() {
    return useBaseModalHook(ModalName.importCategory);
}

export function useCreateSubjectModal() {
    return useBaseModalHook(ModalName.CreateSubject);
}

export function useCreateTagModal() {
    return useBaseModalHook(ModalName.CreateTag);
}

export function useCreateResourceModal() {
    return useBaseModalHook(ModalName.CreateResource);
}

export function useModelConfirmAction(name: ModalName, onConfirm: () => void) {
    const { isConfirm } = useModalSelector().modals[name];

    useEffect(() => {
        if (isConfirm) {
            onConfirm();
        }
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [isConfirm]);
}
