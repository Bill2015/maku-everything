import { invoke } from '@tauri-apps/api/tauri';
import { CategoryResDto, CategoryCreateDto, ExportCategoryDto, CategoryImportDto, CategoryAddRulesResDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace CategoryAPI {
    export function getAll() {
        return invoke<CategoryResDto[]>('get_all_category');
    }

    export function getById(id: string) {
        return invoke<CategoryResDto>('get_category_by_id', { id });
    }

    export function create(data: CategoryCreateDto) {
        return invoke<string>('create_category', { data });
    }

    export function importData(data: CategoryImportDto) {
        return invoke<string>('import_category', { data });
    }

    export function exportData(data: ExportCategoryDto) {
        return invoke<string>('export_category', { data });
    }

    export function getRules(id: string) {
        return invoke<CategoryAddRulesResDto>('get_category_rules', { id });
    }
}
