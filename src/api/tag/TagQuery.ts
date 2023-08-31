import { useQuery } from '@tanstack/react-query';
import { TagAPI } from './TagAPI';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace TagQuery {
    export function useGetAll() {
        const queryfn = () => TagAPI.getAll();

        return useQuery(
            ['subjects'],
            queryfn,
            {
                placeholderData: [],
                initialData:     [],
            },
        );
    }
}
