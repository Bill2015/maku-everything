import { InvokeArgs, invoke } from '@tauri-apps/api/tauri';
import { SubjectResDto, SubjectCreateDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace SubjectAPI {
    export function getAll() {
        return invoke<SubjectResDto[]>('get_all_subject');
    }

    export function getById() {
        return invoke<SubjectResDto>('get_subject_by_id');
    }

    export function create(data: SubjectCreateDto) {
        return invoke<string>('create_subject', data as unknown as InvokeArgs);
    }
}
