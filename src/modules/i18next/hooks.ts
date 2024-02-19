import { TFunction, _Resources } from 'i18next';
import { useTranslation } from 'react-i18next';

export function useValueTranslation<T extends keyof _Resources['common']['Value']>(key: T) {
    const { t, ...funcs } = useTranslation('common', { keyPrefix: `Value.${key}` });
    const tv = t as TFunction<'common', `Value.${T}`>;
    return { tv, ...funcs };
}
