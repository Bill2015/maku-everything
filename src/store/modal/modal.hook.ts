import { useCallback } from 'react';
import { useModelSelector, useModelDispatch } from '../hook';
import { setCreateResourceModelOpen, setCreateSubjectModelOpen, setCreateTagModelOpen } from './modal.slice';

export function useCreateSubjectModel() {
    const { opened } = useModelSelector().createSubject;
    const dispatch = useModelDispatch();

    const open = useCallback(() => {
        dispatch(setCreateSubjectModelOpen(true));
    }, [dispatch]);

    const close = useCallback(() => {
        dispatch(setCreateSubjectModelOpen(false));
    }, [dispatch]);

    return {
        opened, open, close,
    };
}

export function useCreateTagModel() {
    const { opened } = useModelSelector().createTag;
    const dispatch = useModelDispatch();

    const open = useCallback(() => {
        dispatch(setCreateTagModelOpen(true));
    }, [dispatch]);

    const close = useCallback(() => {
        dispatch(setCreateTagModelOpen(false));
    }, [dispatch]);

    return {
        opened, open, close,
    };
}

export function useCreateResourceModel() {
    const { opened } = useModelSelector().createResource;
    const dispatch = useModelDispatch();

    const open = useCallback(() => {
        dispatch(setCreateResourceModelOpen(true));
    }, [dispatch]);

    const close = useCallback(() => {
        dispatch(setCreateResourceModelOpen(false));
    }, [dispatch]);

    return {
        opened, open, close,
    };
}
