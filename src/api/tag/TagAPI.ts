import { InvokeArgs, invoke } from '@tauri-apps/api/tauri';
import { TagCreateDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace TagAPI {
    export function getAll() {
        return invoke('get_all_tag');
    }

    export function getById() {
        return invoke('get_tag_by_id');
    }

    export function create(data: TagCreateDto) {
        return invoke<string>('create_tag', data as unknown as InvokeArgs);
    }
}
