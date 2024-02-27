import { SupportLangsType } from '@modules/i18next';

export interface UpdateConfigDto {
    lang?: SupportLangsType,
}

export interface ConfigResDto {
    lang: SupportLangsType,
}
