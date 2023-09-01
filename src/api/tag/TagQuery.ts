import { useQuery } from '@tanstack/react-query';
import { TagAPI } from './TagAPI';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace TagQuery {
    export function useGetAll() {
        const queryfn = () => TagAPI.getAll();

        return useQuery(
            ['tag'],
            queryfn,
            {
                placeholderData: [],
                initialData:     [],
            },
        );
    }

    export function useGetSubjectTags(categoryId: string, subjectId: string) {
        const queryfn = () => TagAPI.query({
            belong_category: categoryId,
            belong_subject:  subjectId,
        });

        return useQuery(
            ['tag', 'query', categoryId],
            queryfn,
            {
                enabled:         !!categoryId,
                placeholderData: [],
                initialData:     [],
            },
        );
    }
}
