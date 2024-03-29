/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable @typescript-eslint/no-explicit-any */
function replaceBetween(startIndex: number, endIndex: number, original: string, insertion: string) {
    const result = original.substring(0, startIndex) + insertion + original.substring(endIndex);
    return result;
}

export function mockT(i18nKey: string, args?: any) {
    let key = i18nKey;

    while (key.includes('{{')) {
        const startIndex = key.indexOf('{{');
        const endIndex = key.indexOf('}}');

        const currentArg = key.substring(startIndex + 2, endIndex);
        const value = args[currentArg];

        key = replaceBetween(startIndex, endIndex + 2, key, value);
    }

    return key;
}

const i18next: any = jest.createMockFromModule('i18next');
i18next.t = mockT;
i18next.language = 'en';
i18next.changeLanguage = (locale: string) => new Promise(() => {});

export default i18next;
