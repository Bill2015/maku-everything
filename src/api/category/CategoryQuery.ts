import { useQuery } from '@tanstack/react-query';
import { CategoryAPI } from './CategoryAPI';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace CategoryQuery {
    export function useGetAll() {
        const queryfn = () => CategoryAPI.getAll();

        return useQuery(
            ['categories'],
            queryfn,
            {
                placeholderData: [],
                initialData:     [],
            },
        );
    }
}
