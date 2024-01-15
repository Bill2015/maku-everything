import { useCallback, useMemo, useRef, useState } from 'react';
import { TagResDto } from '@api/tag';

import { QueryingNodeProps } from './QueryingNode';
import { InputStatus, InputSymbol } from './enums';
import { ComboboxOptionWithDataProps, InputOption, InputOptionType } from './InputOption';

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

export type InputStatusMechine = {
    /** Status name */
    name: InputStatus;

    /**
     * Available options in this status */
    options: InputSymbol[];

    /**
     * When user select the option's action \
     * Can contain side-effect operate but not recommanded
     * @param val input value
     * @returns Next Status */
    action: (val: string) => InputStatus;
}

/**
 * Define Input Status Mechine \
 * But I wrapped with `useMemo`, maybe in the future will contain side-effect function in action. \
 * Therefore wrapped with `useMemo` instead of pure object */
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
                action:  (val) => ((val === InputSymbol.LeftBracket) ? InputStatus.LeftBracket : InputStatus.Initial),
            },
            {
                name:    InputStatus.TagName,
                options: [InputSymbol.Default, InputSymbol.RightBracket],
                action:  (val) => ((val === InputSymbol.RightBracket) ? InputStatus.Initial : InputStatus.TagName),
            },
            {
                name:    InputStatus.LeftBracket,
                options: [InputSymbol.Default],
                action:  (val) => ((val === InputSymbol.RightBracket) ? InputStatus.Initial : InputStatus.TagName),
            },
        ];
        status.forEach((val) => map.set(val.name, val));
        return map;
    }, []);

    return statusMechine;
};

export type InputStatusHistory = {
    /** Current status */
    status: InputStatus;

    /** Current raw text */
    text: string;

    /** Current displayed querying node */
    display: QueryingNodeProps[];
}

/**
 * Manage the Status History \
 * That can be easily backtrace when user discard his input */
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

/**
 * Major function of complex search
 * @param tags tag data
 * @param searchText search text */
export const useComplexSearch = (tags: TagResDto[], searchText: string) => {
    const inputStateMechine = useInputStatusMechine();
    const [currentInputStatus, setCurrentInputStatus] = useState<InputStatus>(InputStatus.Initial);
    const { popHistory, pushHistory } = useStateHistory();

    // for pass to backend search string
    const [rawText, setRawText] = useState<string>('');

    // for the displaying search querying
    const [queryingNode, setQueryingNode] = useState<QueryingNodeProps[]>([]);

    // Memerized the tags options props
    const tagOptionProps: InputOptionType[] = useMemo(() => (
        tags.map<InputOptionType>((item) => ({
            key:         item.id,
            name:        item.name,
            groupName:   item.subject_name,
            description: item.description,
            value:       `${item.subject_name}:${item.name}`,
            suffix:      `(${item.tagged_count})`,
        }))
    ), [tags]);

    // concat the rawText & querying node from input value
    const newInput = useCallback((value: string, comboxProps: ComboboxOptionWithDataProps) => {
        setRawText((prev) => {
            const lastChar = prev[prev.length - 1];
            if (lastChar === '+' || lastChar === '-') {
                return prev + value;
            }
            return `${prev} ${value}`;
        });
        setQueryingNode((prev) => {
            let newNode: QueryingNodeProps | null = null;
            switch (value) {
            case InputSymbol.Include:
            case InputSymbol.Exclude:
            case InputSymbol.LeftBracket:
            case InputSymbol.RightBracket:
                newNode = { type: 'string', label: value };
                break;
            default:
                newNode = {
                    type:      'tag',
                    label:     comboxProps['data-name']!,
                    groupName: comboxProps['data-groupname']!,
                };
            }
            const lastElement = prev[prev.length - 1];
            if (lastElement && lastElement.type === 'string') {
                // Combine prefix operator with current node
                if (lastElement.label === InputSymbol.Include || lastElement.label === InputSymbol.Exclude) {
                    return [...prev.slice(0, prev.length - 1), { ...newNode, prefix: lastElement.label }];
                }
            }
            return [...prev, newNode];
        });
    }, []);

    // Memerized the selectable options
    const selectableOptions = useMemo(() => {
        const mechine = inputStateMechine.get(currentInputStatus)!;
        return mechine.options
            .reduce<InputOptionType[]>((prev, val) => (
                (val === InputSymbol.Default) ? [...prev, ...tagOptionProps] : [...prev, InputOption.Operators[val]!]
            ), [])
            .filter((item) => item.value.toLowerCase().includes(searchText.toLowerCase().trim()));
    }, [inputStateMechine, currentInputStatus, tagOptionProps, searchText]);

    /**
     * return previous status of search input */
    const backspaceInputSearch: () => InputStatusHistory = useCallback(() => {
        const history = popHistory();
        setCurrentInputStatus(history.status);
        setQueryingNode(history.display);
        setRawText(history.text);
        return history;
    }, [popHistory]);

    /**
     * process the next status of input search */
    const forwardInputSearch = useCallback((val: string, comboxOptionProps: ComboboxOptionWithDataProps) => {
        pushHistory({
            status:  currentInputStatus,
            text:    rawText,
            display: queryingNode,
        });
        // get next status by action
        const nextStatus = inputStateMechine.get(currentInputStatus)!.action(val);
        newInput(val, comboxOptionProps);
        setCurrentInputStatus(nextStatus);
    }, [inputStateMechine, currentInputStatus, queryingNode, rawText, newInput, pushHistory]);

    return {
        options:       selectableOptions,
        displayNode:   queryingNode,
        rawText:       rawText,
        currentStatus: currentInputStatus,
        backspaceInputSearch,
        forwardInputSearch,
    };
};
