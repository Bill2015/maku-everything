// eslint-disable-next-line import/no-extraneous-dependencies
import '@testing-library/jest-dom';
// import { ReactMarkdownProps } from 'react-markdown/lib/ast-to-react';

jest.mock('react-i18next', () => ({
    // this mock makes sure any components using the translate hook can use it without a warning being shown
    useTranslation: () => ({
        t:    (str: string) => str,
        i18n: { changeLanguage: () => new Promise(() => { }) },
    }),
}));

// mock react markdown
// jest.mock('react-markdown', () => ({
//     __esModule: true,
//     default:    ((props: ReactMarkdownProps) => props.children),
// }));
