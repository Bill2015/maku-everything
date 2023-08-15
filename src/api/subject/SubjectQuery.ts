import { useQuery } from '@tanstack/react-query';
import { SubjectAPI } from './SubjectAPI';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace SubjectQuery {
    export function useGetAll() {
        const queryfn = () => SubjectAPI.getAll();

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
