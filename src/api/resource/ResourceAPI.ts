/* eslint-disable camelcase */
import { invoke } from '@tauri-apps/api/tauri';
import { QueryResoruceDto, ResourceCreateDto, ResourceDetailDto, ResourceResDto, ResourceTagOperateDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace ResourceAPI {
    export function getAll() {
        return invoke<ResourceResDto[]>('get_all_resource');
    }

    export function getById(id: string) {
        return invoke<ResourceResDto>('get_resource_by_id', { id: id });
    }

    export function query(queryDto: QueryResoruceDto) {
        return invoke<ResourceResDto[]>('list_resource', { data: queryDto });
    }

    export function queryingByString(q: string, belongCategory?: string) {
        return invoke<ResourceResDto[]>('querying_by_string', { q, belong_category: belongCategory });
    }

    export function getDetail(id: string) {
        return invoke<ResourceDetailDto>('get_resource_detail', { id: id });
    }

    export function create(data: ResourceCreateDto) {
        return invoke<string>('create_resource', { data });
    }

    export function exporeTheFile(filePath: string) {
        return invoke('explore_the_file', { file_path: filePath });
    }

    export function addTag(data: ResourceTagOperateDto) {
        return invoke('add_resource_tag', { data });
    }

    export function removeTag(data: ResourceTagOperateDto) {
        return invoke('remove_resource_tag', { data });
    }
}
