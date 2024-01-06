import { useQuery } from '@tanstack/react-query';
import { SubjectAPI } from './SubjectAPI';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace SubjectQuery {
    export function useGetAll() {
        const queryfn = () => SubjectAPI.getAll();

        return useQuery({
            queryKey:        ['subjects'],
            queryFn:         queryfn,
            placeholderData: [],
            initialData:     [],
        });
    }

    export function useGetByCategory(categoryId: string | null) {
        const queryfn = () => SubjectAPI.query({ belong_category: categoryId! });

        return useQuery({
            queryKey:        ['subjects', categoryId],
            queryFn:         queryfn,
            enabled:         !!categoryId,
            placeholderData: [],
            initialData:     [],
        });
    }
}
