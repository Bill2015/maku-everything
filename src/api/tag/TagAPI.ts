import { InvokeArgs, invoke } from '@tauri-apps/api/tauri';
import { QueryTagDto, TagCreateDto, TagResDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace TagAPI {
    export function getAll() {
        return invoke<TagResDto[]>('get_all_tag');
    }

    export function getById() {
        return invoke<TagResDto[]>('get_tag_by_id');
    }

    export function create(data: TagCreateDto) {
        return invoke<string>('create_tag', data as unknown as InvokeArgs);
    }

    export function query(queryDto: QueryTagDto) {
        return invoke<TagResDto[]>('list_tags', queryDto as unknown as InvokeArgs);
    }
}
