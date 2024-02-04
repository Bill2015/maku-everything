jest.mock('react-i18next', () => ({
    // this mock makes sure any components using the translate hook can use it without a warning being shown
    useTranslation: () => ({
        t:    (str: string) => str,
        i18n: { changeLanguage: () => new Promise(() => { }) },
    }),
}));

jest.mock('@modules/i18next', () => ({ normalizeKey: (key: string) => key }));
