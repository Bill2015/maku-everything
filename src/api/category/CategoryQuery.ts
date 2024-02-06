import { useQuery } from '@tanstack/react-query';
import { CategoryAPI } from './CategoryAPI';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace CategoryQuery {
    export function useGetAll() {
        const queryfn = () => CategoryAPI.getAll();

        return useQuery({
            queryKey:        ['categories'],
            queryFn:         queryfn,
            placeholderData: [],
            initialData:     [],
        });
    }

    export function useGetById(id: string) {
        const queryfn = () => CategoryAPI.getById(id);

        return useQuery({
            queryKey:        ['categories', id],
            queryFn:         queryfn,
            enabled:         !!id,
            placeholderData: null,
            initialData:     null,
        });
    }
}
