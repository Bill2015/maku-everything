import { invoke } from '@tauri-apps/api/tauri';
import { CategoryResDto, CategoryCreateDto, ExportCategoryDto, CategoryImportDto, CategoryMapperRulesResDto, UpdateCategoryMapperRuleDto } from './Dto';

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

    export function updateMapperRule(data: UpdateCategoryMapperRuleDto) {
        return invoke<string>('update_mapper_rule_category', { data });
    }

    export function importData(data: CategoryImportDto) {
        return invoke<string>('import_category', { data });
    }

    export function exportData(data: ExportCategoryDto) {
        return invoke<string>('export_category', { data });
    }

    export function getMapperRules(id: string) {
        return invoke<CategoryMapperRulesResDto>('get_category_mapper_rules', { id });
    }
}
