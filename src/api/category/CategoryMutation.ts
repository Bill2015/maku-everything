import { useMutation } from '@tanstack/react-query';
import { CategoryAPI } from './CategoryAPI';
import { CategoryCreateDto, ExportCategoryDto } from './Dto';
import { CategoryImportDto } from './dto/ImportDto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace CategoryMutation {
    export function useCreate() {
        const mutationFn = (data: CategoryCreateDto) => CategoryAPI.create(data);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useImport() {
        const mutationFn = (data: CategoryImportDto) => CategoryAPI.importData(data);

        return useMutation({ mutationFn: mutationFn });
    }

    export function useExport() {
        const mutationFn = (data: ExportCategoryDto) => CategoryAPI.exportData(data);

        return useMutation({ mutationFn: mutationFn });
    }
}
