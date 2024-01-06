import { useQuery } from '@tanstack/react-query';
import { TagAPI } from './TagAPI';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace TagQuery {
    export function useGetAll() {
        const queryfn = () => TagAPI.getAll();

        return useQuery({
            queryKey:        ['tag'],
            queryFn:         queryfn,
            placeholderData: [],
            initialData:     [],
        });
    }

    export function useGetSubjectTags(subjectId: string | null) {
        const queryfn = () => TagAPI.query({ belong_subject: subjectId! });

        return useQuery({
            queryKey:        ['tag', 'query', subjectId],
            queryFn:         queryfn,
            enabled:         !!subjectId,
            placeholderData: [],
            initialData:     [],
        });
    }
}
