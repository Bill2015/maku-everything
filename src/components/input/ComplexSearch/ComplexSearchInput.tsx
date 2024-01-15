import { useCallback, useMemo, useState } from 'react';
import { Combobox, ComboboxOptionProps, Flex, Group, Input, useCombobox } from '@mantine/core';
import { FaSearch } from 'react-icons/fa';

import { TagResDto } from '@api/tag';
import { InputStatus, InputStatusHistory, useInputStatusMechine, useStateHistory, InputSymbol } from './hooks';
import { InputOption, InputOptionType } from './InputOption';
import { QueryingNode, QueryingNodeProps } from './QueryingNode';

import classes from './ComplexSearchInput.module.scss';

const useComplexSearch = (tags: TagResDto[], searchText: string) => {
    const inputStateMechine = useInputStatusMechine();
    const [currentInputStatus, setCurrentInputStatus] = useState<InputStatus>(InputStatus.Initial);
    const { popHistory, pushHistory } = useStateHistory();

    const [staticText, setStaticText] = useState<string>('');
    const [queryingNode, setQueryingNode] = useState<QueryingNodeProps[]>([]);

    const tagOptionProps: InputOptionType[] = useMemo(() => (
        tags.map<InputOptionType>((item) => ({
            key:         item.id,
            name:        item.name,
            groupName:   item.subject_name,
            description: item.description,
            value:       `${item.subject_name}:${item.name}`,
        }))
    ), [tags]);

    const newInput = useCallback((value: string, comboxProps: ComboboxOptionProps) => {
        setStaticText((prev) => {
            const lastChar = prev[prev.length - 1];
            if (lastChar === '+' || lastChar === '-') {
                return prev + value;
            }
            return `${prev} ${value}`;
        });
        setQueryingNode((prev) => {
            let newNode: QueryingNodeProps | null = null;
            switch (value) {
            case '+':
            case '-':
            case '[':
            case ']':
                newNode = { type: 'string', label: value };
                break;
            default:
                newNode = {
                    type:      'tag',
                    label:     comboxProps['data-name'],
                    groupName: comboxProps['data-groupName'],
                };
            }
            const lastElement = prev[prev.length - 1];
            if (lastElement && lastElement.type === 'string') {
                switch (lastElement.label) {
                case '+':
                case '-':
                    return [...prev.slice(0, prev.length - 1), { ...newNode, prefix: lastElement.label }];
                default:
                }
            }
            return [...prev, newNode];
        });
    }, []);

    const selectableOptions = useMemo(() => {
        const mechine = inputStateMechine.get(currentInputStatus)!;
        return mechine.options
            .reduce<InputOptionType[]>((prev, val) => (
                (val === InputSymbol.Default) ? [...prev, ...tagOptionProps] : [...prev, InputOption.Operators[val]]
            ), [])
            .filter((item) => item.value.toLowerCase().includes(searchText.toLowerCase().trim()))
            .map((item, index) => (
                <InputOption
                    // eslint-disable-next-line react/jsx-props-no-spreading
                    {...item}
                    selected={index === 0}
                />
            ));
    }, [inputStateMechine, currentInputStatus, tagOptionProps, searchText]);

    const backspaceInputSearch: () => InputStatusHistory = useCallback(() => {
        const history = popHistory();
        setCurrentInputStatus(history.status);
        setQueryingNode(history.display);
        setStaticText(history.text);
        return history;
    }, [popHistory]);

    const forwardInputSearch = useCallback((val: string, comboxOptionProps: ComboboxOptionProps) => {
        pushHistory({
            status:  currentInputStatus,
            text:    staticText,
            display: queryingNode,
        });
        const nextStatus = inputStateMechine.get(currentInputStatus)!.action(val);
        newInput(val, comboxOptionProps);
        setCurrentInputStatus(nextStatus);
    }, [inputStateMechine, currentInputStatus, queryingNode, staticText, newInput, pushHistory]);

    return {
        options:       selectableOptions,
        // eslint-disable-next-line react/jsx-props-no-spreading
        displayNode:   queryingNode.map((val) => <QueryingNode {...val} />),
        displayText:   staticText,
        currentStatus: currentInputStatus,
        backspaceInputSearch,
        forwardInputSearch,
    };
};

export interface ComplexSearchInputProps {
    tags: TagResDto[]
}

export function ComplexSearchInput(props: ComplexSearchInputProps) {
    const { tags } = props;
    const combobox = useCombobox({ onDropdownClose: () => combobox.resetSelectedOption() });
    const [searchText, setSearchText] = useState<string>('');
    const { options, displayNode, displayText, backspaceInputSearch, forwardInputSearch } = useComplexSearch(tags, searchText);

    const handleOptionSubmit = useCallback((val: string, comboxOptionProps: ComboboxOptionProps) => {
        forwardInputSearch(val, comboxOptionProps);
        combobox.selectFirstOption();
        setSearchText('');
    }, [combobox, forwardInputSearch]);

    return (
        <Combobox
            position="bottom"
            store={combobox}
            onOptionSubmit={handleOptionSubmit}
        >
            <Combobox.Target>
                <Group classNames={{ root: classes.searchRoot }}>
                    <Flex gap="sm">
                        {displayNode}
                    </Flex>
                    <Input
                        value={searchText}
                        placeholder="search here..."
                        classNames={{ wrapper: classes.inputWrapper, input: classes.input }}
                        onChange={(e) => {
                            setSearchText(e.currentTarget.value);
                        }}
                        onKeyDown={(e) => {
                            if (e.key === 'Backspace' && searchText === '') {
                                backspaceInputSearch();
                            }
                        }}
                        onClick={() => combobox.toggleDropdown()}
                        rightSection={<FaSearch />}
                    />
                </Group>
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Options mah="50dvh" style={{ overflowY: 'auto' }}>
                    {
                        (options.length > 0)
                            ? options
                            : <Combobox.Empty>Nothing found</Combobox.Empty>
                    }
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
}
