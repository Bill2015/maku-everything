import { invoke } from '@tauri-apps/api/tauri';
import { ConfigResDto, UpdateConfigDto } from './Dto';

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace ConfigAPI {
    export function get() {
        return invoke<ConfigResDto>('get_config');
    }

    export function update(data: UpdateConfigDto) {
        return invoke<string>('update_config', { data });
    }
}
