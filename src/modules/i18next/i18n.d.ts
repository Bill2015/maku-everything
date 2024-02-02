/* eslint-disable camelcase */
/* eslint-disable import/extensions */
import { resources, defaultLang } from './i18next';

declare module 'i18next' {
    interface CustomTypeOptions {
        defaultNS: typeof defaultLang.key;
        resources: typeof resources['enUS'];
    }
}
