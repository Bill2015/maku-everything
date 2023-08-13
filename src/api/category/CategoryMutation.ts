import { useMutation } from '@tanstack/react-query';
import { CategoryAPI } from './CategoryAPI';
import { CategoryCreateDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace CategoryMutation {
    export function useCreate() {
        const mutationFn = (data: CategoryCreateDto) => CategoryAPI.create(data);

        return useMutation({ mutationFn: mutationFn });
    }
}
