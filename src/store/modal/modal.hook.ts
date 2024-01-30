import { useCallback } from 'react';
import { useModelSelector, useModelDispatch } from '../hook';
import {
    ModalName,
    setModalOpenStatus,
} from './modal.slice';

function useBaseModalHook(name: ModalName): [boolean, { open: () => void, close: () => void }] {
    const { opened } = useModelSelector().modals[name];
    const dispatch = useModelDispatch();

    const open = useCallback(() => {
        dispatch(setModalOpenStatus({ name: name, open: true }));
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch]);

    const close = useCallback(() => {
        dispatch(setModalOpenStatus({ name: name, open: false }));
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch]);

    return [opened, { open, close }];
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
