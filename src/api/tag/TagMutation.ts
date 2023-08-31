import { useMutation } from '@tanstack/react-query';
import { TagAPI } from './TagAPI';
import { TagCreateDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace TagMutation {
    export function useCreate() {
        const mutationFn = (data: TagCreateDto) => TagAPI.create(data);

        return useMutation({ mutationFn: mutationFn });
    }
}
