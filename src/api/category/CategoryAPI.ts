import { invoke } from '@tauri-apps/api/tauri';
import { CategoryResDto, CategoryCreateDto } from './Dto';
import { CategoryImportDto } from './dto/ImportDto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace CategoryAPI {
    export function getAll() {
        return invoke<CategoryResDto[]>('get_all_category');
    }

    export function getById() {
        return invoke<CategoryResDto>('get_category_by_id');
    }

    export function create(data: CategoryCreateDto) {
        return invoke<string>('create_category', { data });
    }

    export function importData(data: CategoryImportDto) {
        return invoke<string>('import_category', { data });
    }
}
