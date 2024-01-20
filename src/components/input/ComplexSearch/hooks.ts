import { useCallback, useMemo, useRef, useState } from 'react';
import * as lodash from 'lodash';
import { TagResDto } from '@api/tag';

import { QueryingNodeProps } from './QueryingNode';
import { SearchStatus, InputSymbol } from './enums';
import { ComboboxOptionWithDataProps, InputOption, InputOptionType } from './InputOption';

// Initial =| PrefixOperator

// PrefixOperator =| TagName
//                 | LeftGroupBracket

// TagName =| Initial
//          | TagName
//          | RightGroupBracket
//          | LeftAttrBracket

// LeftAttrBracket =| Attribute

// LeftGroupBracket =| TagName
//                   | Functional Tag

// +AI => 只顯示包含 AI 標籤的資源
// -AI => 不顯示包含 AI 標籤的資源
// +AI +Python => 顯示包含 AI 與 Python 標籤的資源
// +(AI | Python) +Javascript => 顯示一定要有 Javascript 但是可能包含 AI 或 Python 標籤的資源
// -(AI | Python) +Javascript === -AI -Python +Javascript;
// -(AI & Python) +Javascript => 顯示一定要有 Javascript 但是不能同時包含 AI 與 Python 標籤的資源
// +(AI & Python) +Javascript === +AI +Python +Javascript;

type InputStatus = {
    status: SearchStatus;

    inGroup: boolean;

    inAttribute: boolean;
}

export type InputStatusMechine = {
    /** Status name */
    name: SearchStatus;

    /**
     * Available options in this status
     * @param prevStatus previous status
     * @returns selectable array */
    options: (prevStatus: InputStatus) => InputSymbol[];

    /**
     * When user select the option's action \
     * Can contain side-effect operate but not recommanded
     * @param val input value
     * @param prevStatus previous status
     * @returns Next Status */
    action: (val: string, prevStatus: InputStatus) => InputStatus;
}

/**
 * Define Input Status Mechine */
const STATUS_MAP: Map<SearchStatus, InputStatusMechine> = (() => {
    const map = new Map<SearchStatus, InputStatusMechine>();
    const status: InputStatusMechine[] = [
        {
            name:    SearchStatus.Initial,
            options: () => [InputSymbol.Include, InputSymbol.Exclude],
            action:  (_, prevStatus) => ({ ...prevStatus, status: SearchStatus.PrefixOperator }),
        },
        {
            name:    SearchStatus.PrefixOperator,
            options: () => [InputSymbol.Default, InputSymbol.LeftGroupBracket],
            action:  (val, prevStatus) => ({
                ...prevStatus,
                status:  ((val === InputSymbol.LeftGroupBracket) ? SearchStatus.Group : SearchStatus.TagName),
                inGroup: (val === InputSymbol.LeftGroupBracket),
            }),
        },
        {
            name:    SearchStatus.TagName,
            options: (prevStatus) => {
                const option = [];
                if (prevStatus.inGroup) {
                    option.push(InputSymbol.Default);
                    option.push(InputSymbol.RightGroupBracket);
                }
                else {
                    option.push(InputSymbol.Include);
                    option.push(InputSymbol.Exclude);
                }
                if (prevStatus.inAttribute === false) {
                    option.push(InputSymbol.LeftAttrBracket);
                }
                return option;
            },
            action: (val, prevStatus) => {
                if (InputSymbol.isPrefix(val)) {
                    return {
                        ...prevStatus, status: SearchStatus.PrefixOperator, inGroup: false,
                    };
                }
                if (val === InputSymbol.RightGroupBracket) {
                    return {
                        ...prevStatus, status: SearchStatus.Initial, inGroup: false,
                    };
                }
                if (val === InputSymbol.LeftAttrBracket) {
                    return {
                        ...prevStatus, status: SearchStatus.Attribute, inAttribute: true,
                    };
                }
                if (prevStatus.inGroup) {
                    return { ...prevStatus, status: SearchStatus.TagName };
                }
                return { ...prevStatus, status: SearchStatus.Initial };
            },
        },
        {
            name:    SearchStatus.Group,
            options: () => [InputSymbol.Default],
            action:  (val, prevStatus) => ({
                ...prevStatus,
                status: SearchStatus.TagName,
            }),
        },
        {
            name:    SearchStatus.Attribute,
            options: () => [],
            action:  (val, prevStatus) => ({
                ...prevStatus,
                inAttribute: false,
                status:      (prevStatus.inGroup) ? SearchStatus.TagName : SearchStatus.Initial,
            }),
        },
    ];
    status.forEach((val) => map.set(val.name, val));
    return map;
})();

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
                status: {
                    status: SearchStatus.Initial, inGroup: false, inAttribute: false,
                },
                display: [],
                text:    '',
            };
        }
        return statusStackRef.current.pop()!;
    }, []);

    const pushHistory = useCallback((history: InputStatusHistory) => {
        statusStackRef.current.push(history);
    }, []);

    const clearHistory = useCallback(() => {
        statusStackRef.current = [];
    }, []);

    return {
        popHistory,
        pushHistory,
        clearHistory,
    };
};

/**
 * Major function of complex search
 * @param tags tag data
 * @param searchText search text */
export const useComplexSearch = (tags: TagResDto[], searchText: string) => {
    const [currentInputStatus, setCurrentInputStatus] = useState<InputStatus>({
        status:      SearchStatus.Initial,
        inGroup:     false,
        inAttribute: false,
    });
    const { popHistory, pushHistory, clearHistory } = useStateHistory();

    // for the displaying search querying
    const [queryingNode, setQueryingNode] = useState<QueryingNodeProps[]>([]);

    // Memerized the tags options props
    const tagOptionProps: InputOptionType[] = useMemo(() => (
        tags.map<InputOptionType>((item) => ({
            key:           item.id,
            name:          item.name,
            groupName:     item.subject_name,
            description:   item.description,
            value:         `${item.subject_name}:${item.name}`,
            suffix:        `(${item.tagged_count})`,
            attributeType: 'none',
        }))
    ), [tags]);

    // concat the rawText & querying node from input value
    const newInput = useCallback((value: string, comboxProps: ComboboxOptionWithDataProps | null) => {
        setQueryingNode((prev) => {
            let newNode: QueryingNodeProps = {
                type:   'tag',
                prefix: '',
                suffix: '',
                label:  '',
            };
            if (!comboxProps) {
                newNode = lodash.merge<QueryingNodeProps, QueryingNodeProps>(newNode, { type: 'attribute', label: value });
            }
            else if (InputSymbol.isValid(value)) {
                newNode = lodash.merge<QueryingNodeProps, QueryingNodeProps>(newNode, { type: 'operator', label: value });
            }
            else {
                newNode = lodash.merge<QueryingNodeProps, QueryingNodeProps>(newNode, {
                    type: 'tag', label: comboxProps['data-name']!, groupName: comboxProps['data-groupname']!,
                });
            }

            const lastElement = prev[prev.length - 1];
            if (!lastElement) {
                return [newNode];
            }
            // Combine prefix operator with current node
            if (InputSymbol.isPrefix(lastElement.label)) {
                return [...prev.slice(0, prev.length - 1), { ...newNode, prefix: lastElement.label }];
            }
            // Combine suffix with attribute bracket
            if (lastElement.type === 'tag' && newNode.label === '{') {
                return [...prev.slice(0, prev.length - 1), { ...lastElement, suffix: newNode.label }];
            }
            // Combine suffix with attribute value
            if (lastElement.type === 'tag' && newNode.type === 'attribute') {
                return [...prev.slice(0, prev.length - 1), { ...lastElement, suffix: `${lastElement.suffix}${newNode.label}}` }];
            }
            return [...prev, newNode];
        });
    }, []);

    /**
     * This text is for pass to backend and search used  */
    const rawText = useMemo(() => (
        queryingNode.map((item) => {
            if (item.type === 'tag') {
                return `${item.prefix}"${item.groupName}:${item.label}"${item.suffix}`;
            }
            return `${item.prefix}${item.label}${item.suffix}`;
        }).join(' ')
    ), [queryingNode]);

    // Memerized the selectable options
    const selectableOptions = useMemo(() => {
        const mechine = STATUS_MAP.get(currentInputStatus.status)!;
        return mechine.options(currentInputStatus)
            .reduce<InputOptionType[]>((prev, val) => (
                (val === InputSymbol.Default) ? [...prev, ...tagOptionProps, ...InputOption.FunctionalTags] : [...prev, InputOption.Operators[val]!]
            ), [])
            .filter((item) => item.value.toLowerCase().includes(searchText.toLowerCase().trim()));
    }, [currentInputStatus, tagOptionProps, searchText]);

    /**
     * return previous status of search input */
    const backspaceInputSearch: () => InputStatusHistory = useCallback(() => {
        const history = popHistory();
        setCurrentInputStatus(history.status);
        setQueryingNode(history.display);
        return history;
    }, [popHistory]);

    /**
     * process the next status of input search */
    const forwardInputSearch = useCallback((val: string, comboxOptionProps: ComboboxOptionWithDataProps | null) => {
        if (!comboxOptionProps && currentInputStatus.status !== SearchStatus.Attribute) {
            return;
        }
        pushHistory({
            status:  currentInputStatus,
            text:    rawText,
            display: queryingNode,
        });
        // get next status by action
        const nextStatus = STATUS_MAP.get(currentInputStatus.status)!.action(val, currentInputStatus);
        newInput(val, comboxOptionProps);
        setCurrentInputStatus(nextStatus);
    }, [currentInputStatus, queryingNode, rawText, newInput, pushHistory]);

    /**
     * Clear the search */
    const clearSearch = useCallback(() => {
        setCurrentInputStatus({
            status: SearchStatus.Initial, inGroup: false, inAttribute: false,
        });
        setQueryingNode([]);
        clearHistory();
    }, [clearHistory]);

    return {
        options:       selectableOptions,
        displayNode:   queryingNode,
        rawText:       rawText,
        currentStatus: currentInputStatus,
        backspaceInputSearch,
        forwardInputSearch,
        clearSearch,
    };
};
