import { useMutation } from '@tanstack/react-query';
import { ResourceAPI } from './ResourceAPI';
import { ResourceCreateDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace ResourceMutation {
    export function useCreate() {
        const mutationFn = (data: ResourceCreateDto) => ResourceAPI.create(data);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useExporeFile() {
        const mutationFn = (filePath: string) => ResourceAPI.exporeTheFile(filePath);

        return useMutation({ mutationFn: mutationFn });
    }
}
