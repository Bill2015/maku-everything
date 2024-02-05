import { useMemo, useState } from 'react';

const defaultValue = [' ', '\\', '/', '-'];
export function useSeparatorText(text: string) {
    const [separators, setSeparators] = useState<string[]>(defaultValue);

    const separateResult = useMemo(() => {
        if (separators.length <= 0) {
            return [text];
        }

        const regexString = separators.map((val) => {
            switch (val) {
            case '\\':
            case '[':
                return `\\${val}`;
            default:
                return val;
            }
        }).join('|');

        const regex = new RegExp(regexString, 'gi');

        return [...new Set(text.split(regex))]
            .filter((val) => !!val)
            .map((val) => val.trim());
    }, [separators, text]);

    const reset = () => setSeparators(defaultValue);

    return {
        separators, setSeparators, separateResult, reset,
    };
}
