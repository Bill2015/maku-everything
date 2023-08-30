import { useQuery } from '@tanstack/react-query';
import { ResourceAPI } from './ResourceAPI';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace ResourceQuery {
    export function useGetAll() {
        const queryfn = () => ResourceAPI.getAll();

        return useQuery(
            ['resource'],
            queryfn,
            {
                placeholderData: [],
                initialData:     [],
            },
        );
    }

    export function useGetById(id: string) {
        const queryfn = () => ResourceAPI.getById(id);

        return useQuery(
            ['resurce', id],
            queryfn,
            {
                placeholderData: null,
                initialData:     null,
            },
        );
    }
}
