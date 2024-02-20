import { useCallback, useMemo, useRef, useState } from 'react';
import lodash from 'lodash';
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

    groupStatus: 'none' | 'in-include' | 'in-exclude';

    value: string;

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
            action:  (val, prevStatus) => ({
                ...prevStatus,
                value:  val,
                status: SearchStatus.PrefixOperator,
            }),
        },
        {
            name:    SearchStatus.PrefixOperator,
            options: () => [InputSymbol.Default, InputSymbol.LeftGroupBracket],
            action:  (val, prevStatus) => ({
                ...prevStatus,
                value:       val,
                status:      ((val === InputSymbol.LeftGroupBracket) ? SearchStatus.Group : SearchStatus.TagName),
                groupStatus: (() => {
                    if (val === InputSymbol.LeftGroupBracket) {
                        // determin group type
                        return (prevStatus.value === InputSymbol.Include) ? 'in-include' : 'in-exclude';
                    }
                    return 'none';
                })(),
            }),
        },
        {
            name:    SearchStatus.TagName,
            options: (prevStatus) => {
                const option = [];
                if (prevStatus.groupStatus !== 'none') {
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
                const newStatus: InputStatus = { ...prevStatus, value: val };
                if (InputSymbol.isPrefix(val)) {
                    return {
                        ...newStatus, status: SearchStatus.PrefixOperator, groupStatus: 'none',
                    };
                }
                if (val === InputSymbol.RightGroupBracket) {
                    return {
                        ...newStatus, status: SearchStatus.Initial, groupStatus: 'none',
                    };
                }
                if (val === InputSymbol.LeftAttrBracket) {
                    return {
                        ...newStatus, status: SearchStatus.Attribute, inAttribute: true,
                    };
                }
                if (prevStatus.groupStatus !== 'none') {
                    return { ...newStatus, status: SearchStatus.TagName };
                }
                return { ...newStatus, status: SearchStatus.Initial };
            },
        },
        {
            name:    SearchStatus.Group,
            options: () => [InputSymbol.Default],
            action:  (val, prevStatus) => ({
                ...prevStatus,
                value:  val,
                status: SearchStatus.TagName,
            }),
        },
        {
            name:    SearchStatus.Attribute,
            options: () => [],
            action:  (val, prevStatus) => ({
                ...prevStatus,
                value:       val,
                inAttribute: false,
                status:      (prevStatus.groupStatus !== 'none') ? SearchStatus.TagName : SearchStatus.Initial,
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

    /** input text array */
    inputs: string[];

    /** Current displayed querying node */
    display: QueryingNodeProps[];
}

/**
 * Manage the Status History \
 * That can be easily backtrace when user discard his input */
export const useStateHistory = () => {
    const statusStackRef = useRef<InputStatusHistory[]>([]);

    const pop = useCallback<() => InputStatusHistory >(() => {
        if (statusStackRef.current.length <= 0) {
            return {
                status: {
                    status:      SearchStatus.Initial,
                    groupStatus: 'none',
                    inAttribute: false,
                    value:       '',
                },
                inputs:  [],
                display: [],
                text:    '',
            };
        }
        return statusStackRef.current.pop()!;
    }, []);

    const peekCurrent = useCallback(() => statusStackRef.current.at(-1), []);

    const isEmpty = useCallback(() => statusStackRef.current.length <= 0, []);

    const push = useCallback((history: InputStatusHistory) => {
        statusStackRef.current.push(history);
    }, []);

    const clear = useCallback(() => {
        statusStackRef.current = [];
    }, []);

    return useMemo(() => ({
        pop,
        push,
        clear,
        isEmpty,
        peekCurrent,
    }), [pop, push, clear, isEmpty, peekCurrent]);
};

/**
 * Major function of complex search
 * @param tags tag data
 * @param searchText search text */
export const useComplexSearch = (tags: TagResDto[], searchText: string) => {
    const [currentInputStatus, setCurrentInputStatus] = useState<InputStatus>({
        status:      SearchStatus.Initial,
        groupStatus: 'none',
        inAttribute: false,
        value:       '',
    });
    const historyManager = useStateHistory();

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
    const newInput = useCallback((value: string, comboxProps: ComboboxOptionWithDataProps | null, status: InputStatus) => {
        setQueryingNode((prev) => {
            let newNode: QueryingNodeProps = {
                type:   'tag',
                prefix: '',
                suffix: '',
                label:  '',
            };
            let concatNode: QueryingNodeProps | null = null;
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
                concatNode = { type: 'display-only', label: (status.groupStatus === 'in-include') ? 'or' : 'and' };
            }

            const lastElement = prev[prev.length - 1];
            if (!lastElement) {
                return [newNode];
            }
            const prevSlice = prev.slice(0, prev.length - 1);
            // Combine prefix operator with current node
            if (InputSymbol.isPrefix(lastElement.label)) {
                return [...prevSlice, { ...newNode, prefix: lastElement.label }];
            }
            // Combine suffix with attribute bracket
            if (lastElement.type === 'tag' && newNode.label === '{') {
                return [...prevSlice, { ...lastElement, suffix: newNode.label }];
            }
            // Combine suffix with attribute value
            if (lastElement.type === 'tag' && newNode.type === 'attribute') {
                return [...prevSlice, { ...lastElement, suffix: `${lastElement.suffix}${newNode.label}}` }];
            }
            // if in the group tag name, add concat node
            if (concatNode && lastElement.type !== 'operator') {
                return [...prev, concatNode, newNode];
            }
            // padding a empty node after the ']'
            if (newNode.label === ']') {
                return [...prev, newNode, { label: '', type: 'display-only' }];
            }
            return [...prev, newNode];
        });
    }, []);

    /**
     * This text is for pass to backend and search used  */
    const rawText = useMemo(() => (
        queryingNode
            .filter((item) => item.type !== 'display-only')
            .map((item) => {
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
        const history = historyManager.pop();
        setCurrentInputStatus(history.status);
        setQueryingNode(history.display);
        return history;
    }, [historyManager]);

    /**
     * process the next status of input search
     * @param inputVal original input value from search input
     * @param optionVal value from option selected
     * @param comboxOptionProps selected comboxx props
     * @returns `true` if forward successful, `false` then otherwise */
    const forwardInputSearch = useCallback((inputVal: string, optionVal: string, comboxOptionProps: ComboboxOptionWithDataProps | null) => {
        if (!comboxOptionProps && currentInputStatus.status !== SearchStatus.Attribute) {
            return false;
        }
        historyManager.push({
            status:  currentInputStatus,
            text:    rawText,
            inputs:  (historyManager.isEmpty() ? [inputVal] : [...historyManager.peekCurrent()!.inputs, inputVal]),
            display: queryingNode,
        });
        // get next status by action
        const nextStatus = STATUS_MAP.get(currentInputStatus.status)!.action(optionVal, currentInputStatus);
        newInput(optionVal, comboxOptionProps, currentInputStatus);
        setCurrentInputStatus(nextStatus);
        return true;
    }, [currentInputStatus, queryingNode, rawText, newInput, historyManager]);

    /**
     * Clear the search */
    const clearSearch = useCallback(() => {
        setCurrentInputStatus({
            status: SearchStatus.Initial, groupStatus: 'none', inAttribute: false, value: '',
        });
        setQueryingNode([]);
        historyManager.clear();
    }, [historyManager]);

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
