import { InvokeArgs, invoke } from '@tauri-apps/api/tauri';
import { ResourceCreateDto, ResourceResDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace ResourceAPI {
    export function getAll() {
        return invoke<ResourceResDto[]>('get_all_resource');
    }

    export function getById(id: string) {
        return invoke<ResourceResDto>('get_resource_by_id', { id: id });
    }

    export function create(data: ResourceCreateDto) {
        return invoke<string>('create_resource', data as unknown as InvokeArgs);
    }
}
