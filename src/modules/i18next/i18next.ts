/* eslint-disable camelcase */
/* eslint-disable import/extensions */
import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';

// Language Files
import commonEN_US from '@assets/locales/common/en-US.json';
import pageCommonEN_US from '@assets/locales/pages/common/en-US.json';
import pageCategorylistEN_US from '@assets/locales/pages/category-list/en-US.json';
import pageResourcelistEN_US from '@assets/locales/pages/resource-list/en-US.json';
import pageResourceDetailEN_US from '@assets/locales/pages/resource-detail/en-US.json';
import pageResourceAddEN_US from '@assets/locales/pages/resource-add/en-US.json';
import modalTagCreateEN_US from '@assets/locales/modal/create-tag/en-US.json';

import commonZH_TW from '@assets/locales/common/zh-TW.json';
import pageCommonZH_TW from '@assets/locales/pages/common/zh-TW.json';
import pageCategorylistZH_TW from '@assets/locales/pages/category-list/zh-TW.json';
import pageResourcelistZH_TW from '@assets/locales/pages/resource-list/zh-TW.json';
import pageResourceDetailZH_TW from '@assets/locales/pages/resource-detail/zh-TW.json';
import pageResourceAddZH_TW from '@assets/locales/pages/resource-add/zh-TW.json';
import modalTagCreateZH_TW from '@assets/locales/modal/create-tag/zh-TW.json';

export type SupportLangsType = 'enUS' | 'zhTW';

export type Language = { key: SupportLangsType, displayName: string }

export const SupportLangs: {[key in SupportLangsType]: Language} = {
    enUS: {
        key:         'enUS',
        displayName: 'en-US (America)',
    },
    zhTW: {
        key:         'zhTW',
        displayName: 'zh-TW (台灣)',
    },
};

export const defaultLang: Language = SupportLangs.enUS;

export const resources = {
    [SupportLangs.enUS.key]: {
        common: commonEN_US,
        pages:  {
            Common:         pageCommonEN_US,
            CategoryList:   pageCategorylistEN_US,
            resourceList:   pageResourcelistEN_US,
            resourceDetail: pageResourceDetailEN_US,
            resourceAdd:    pageResourceAddEN_US,
        },
        modal: { createTag: modalTagCreateEN_US },
    },
    [SupportLangs.zhTW.key]: {
        common: commonZH_TW,
        pages:  {
            Common:         pageCommonZH_TW,
            CategoryList:   pageCategorylistZH_TW,
            resourceList:   pageResourcelistZH_TW,
            resourceDetail: pageResourceDetailZH_TW,
            resourceAdd:    pageResourceAddZH_TW,
        },
        modal: { createTag: modalTagCreateZH_TW },
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
            'pages',
        ],
        // default namespace
        defaultNS:     'common',
        // default language
        lng:           SupportLangs.enUS.key,
        fallbackLng:   SupportLangs.enUS.key,
        keySeparator:  '.',
        supportedLngs: [
            SupportLangs.enUS.key,
            SupportLangs.zhTW.key,
        ],
        interpolation: { escapeValue: false },
    });

export default i18next;
