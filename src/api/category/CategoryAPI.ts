import { invoke } from '@tauri-apps/api/tauri';
import { CategoryResDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace CategoryAPI {
    export function getAll() {
        return invoke<CategoryResDto[]>('get_all_category');
    }

    export function getById() {
        return invoke<CategoryResDto>('get_category_by_id');
    }
}
