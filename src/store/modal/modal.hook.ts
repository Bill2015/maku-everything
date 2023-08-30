import { useCallback } from 'react';
import { useModelSelector, useModelDispatch } from '../hook';
import { setCreateSubjectModelOpen } from './modal.slice';

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
