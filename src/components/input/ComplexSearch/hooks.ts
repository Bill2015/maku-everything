import { useCallback, useMemo, useRef } from 'react';
import { QueryingNodeProps } from './QueryingNode';

// Initial =| PrefixOperator

// PrefixOperator =| TagName
//                 | Left Bracket

// TagName =| Initial
//          | TagName
//          | Right Bracket

// LeftBracket =| TagName

// +AI => 只顯示包含 AI 標籤的資源
// -AI => 不顯示包含 AI 標籤的資源
// +AI +Python => 顯示包含 AI 與 Python 標籤的資源
// +(AI | Python) +Javascript => 顯示一定要有 Javascript 但是可能包含 AI 或 Python 標籤的資源
// -(AI | Python) +Javascript === -AI -Python +Javascript;
// -(AI & Python) +Javascript => 顯示一定要有 Javascript 但是不能同時包含 AI 與 Python 標籤的資源
// +(AI & Python) +Javascript === +AI +Python +Javascript;

// eslint-disable-next-line no-shadow
export enum InputSymbol {
    Default = 'default',
    Include = '+',
    Exclude = '-',
    LeftBracket = '[',
    RightBracket = ']',
}

// eslint-disable-next-line no-shadow
export enum InputStatus {
    Initial,
    PrefixOperator, // -, +
    TagName, // tag, left bracket
    LeftBracket,
}

export type InputStatusMechine = {
    name: InputStatus;

    options: InputSymbol[];

    action: (val: string) => InputStatus;
}

export const useInputStatusMechine = () => {
    const statusMechine: Map<InputStatus, InputStatusMechine> = useMemo(() => {
        const map = new Map<InputStatus, InputStatusMechine>();
        const status: InputStatusMechine[] = [
            {
                name:    InputStatus.Initial,
                options: [InputSymbol.Include, InputSymbol.Exclude],
                action:  (_val) => InputStatus.PrefixOperator,
            },
            {
                name:    InputStatus.PrefixOperator,
                options: [InputSymbol.Default, InputSymbol.LeftBracket],
                action:  (val) => ((val === '[') ? InputStatus.LeftBracket : InputStatus.Initial),
            },
            {
                name:    InputStatus.TagName,
                options: [InputSymbol.Default, InputSymbol.RightBracket],
                action:  (val) => ((val === ']') ? InputStatus.Initial : InputStatus.TagName),
            },
            {
                name:    InputStatus.LeftBracket,
                options: [InputSymbol.Default],
                action:  (val) => ((val === ']') ? InputStatus.Initial : InputStatus.TagName),
            },
        ];
        status.forEach((val) => map.set(val.name, val));
        return map;
    }, []);

    return statusMechine;
};

export type InputStatusHistory = {
    status: InputStatus;
    text: string;
    display: QueryingNodeProps[];
}

export const useStateHistory = () => {
    const statusStackRef = useRef<InputStatusHistory[]>([]);

    const popHistory = useCallback(() => {
        if (statusStackRef.current.length <= 0) {
            return {
                status:  InputStatus.Initial,
                display: [],
                text:    '',
            };
        }
        return statusStackRef.current.pop()!;
    }, []);

    const pushHistory = useCallback((history: InputStatusHistory) => {
        statusStackRef.current.push(history);
    }, []);

    return {
        popHistory,
        pushHistory,
    };
};
