import { useMutation } from '@tanstack/react-query';
import { SubjectAPI } from './SubjectAPI';
import { SubjectCreateDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace SubjectMutation {
    export function useCreate() {
        const mutationFn = (data: SubjectCreateDto) => SubjectAPI.create(data);

        return useMutation({ mutationFn: mutationFn });
    }
}
