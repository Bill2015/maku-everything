/* eslint-disable camelcase */
/* eslint-disable import/extensions */
import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';

// Language Files
import commonEN_US from '@assets/locales/common/en-US.json';
import pageCommonEN_US from '@assets/locales/pages/common/en-US.json';
import pageCategorylistEN_US from '@assets/locales/pages/category-list/en-US.json';
import pageResourcelistEN_US from '@assets/locales/pages/resource-list/en-US.json';

import commonZH_TW from '@assets/locales/common/zh-TW.json';
import pageCommonZH_TW from '@assets/locales/pages/common/zh-TW.json';
import pageCategorylistZH_TW from '@assets/locales/pages/category-list/zh-TW.json';
import pageResourcelistZH_TW from '@assets/locales/pages/resource-list/zh-TW.json';

export type SupportLangsType = 'enUS' | 'zhTW';

export const defaultNS: SupportLangsType = 'enUS';

export const SupportLangs: {[key in SupportLangsType]: SupportLangsType} = {
    enUS: 'enUS',
    zhTW: 'zhTW',
};

export const resources = {
    [SupportLangs.enUS]: {
        common: commonEN_US,
        pages:  {
            Common:       pageCommonEN_US,
            CategoryList: pageCategorylistEN_US,
            resourceList: pageResourcelistEN_US,
        },
    },
    [SupportLangs.zhTW]: {
        common: commonZH_TW,
        pages:  {
            Common:       pageCommonZH_TW,
            CategoryList: pageCategorylistZH_TW,
            resourceList: pageResourcelistZH_TW,
        },
    },
} as const;

/**
 * For Dynamic key
 * @SeeAlso
 * https://stackoverflow.com/questions/70914886/react-i18n-t-function-doesnt-accept-string-variables-typescript-no-over/71896191#71896191
 * @param key dynamic key
 * @returns TemplateStringsArray */
export const normalizeKey = (key: string) => key as unknown as TemplateStringsArray;

i18next
    .use(initReactI18next)
    .init({
        resources,
        // Namespace
        ns: [
            'common',
        ],
        // default namespace
        defaultNS:     'common',
        // default language
        lng:           SupportLangs.enUS,
        fallbackLng:   SupportLangs.enUS,
        keySeparator:  '.',
        supportedLngs: [
            SupportLangs.enUS,
            SupportLangs.zhTW,
        ],
        interpolation: { escapeValue: false },
    });

export default i18next;
