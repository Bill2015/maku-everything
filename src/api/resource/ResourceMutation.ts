import { useMutation } from '@tanstack/react-query';
import { ResourceAPI } from './ResourceAPI';
import { ResourceCreateDto, ResourceRenameFileDto, ResourceTagOperateDto, ResourceUpdateDto, ResourceUpdateTagDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace ResourceMutation {
    export function useCreate() {
        const mutationFn = (data: ResourceCreateDto) => ResourceAPI.create(data);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useUpdate() {
        const mutationFn = (data: ResourceUpdateDto) => ResourceAPI.update(data);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useExporeFile() {
        const mutationFn = (filePath: string) => ResourceAPI.exporeTheFile(filePath);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useRenameFile() {
        const mutationFn = (data: ResourceRenameFileDto) => ResourceAPI.renameTheFile(data);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useAddTag() {
        const mutationFn = (data: ResourceTagOperateDto) => ResourceAPI.addTag(data);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useRemoveTag() {
        const mutationFn = (data: ResourceTagOperateDto) => ResourceAPI.removeTag(data);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useUpdateTag() {
        const mutationFn = (data: ResourceUpdateTagDto) => ResourceAPI.updateTag(data);

        return useMutation({ mutationFn: mutationFn });
    }
}
