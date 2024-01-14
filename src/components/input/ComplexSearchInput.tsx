import { ReactNode, Reducer, useCallback, useMemo, useRef, useState } from 'react';
import { Badge, Combobox, ComboboxOptionProps, Flex, Group, Input, Stack, Text, useCombobox } from '@mantine/core';
import { FaSearch } from 'react-icons/fa';

import { TagResDto } from '@api/tag';

import classes from './ComplexSearchInput.module.scss';

type InputOptionType = {
    key: string;

    description: string;

    name: string;

    value: string;

    groupName: string;

    isOperator?: boolean;
}

export interface InputOptionProps extends InputOptionType, Omit<ComboboxOptionProps, 'key'> { }

export function InputOption(props: InputOptionProps) {
    const { description, name, groupName, ...optionProps } = props;

    return (
        <Combobox.Option
            aria-description={description}
            data-groupName={groupName}
            data-name={name}
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...optionProps}
        >
            <Stack gap={0} p={0}>
                <Text component="span" h="0.75em" fz="0.75em" opacity="0.5">{groupName}</Text>
                <Text component="span" h="1.1em">
                    {name}
                    <Text component="span" pl={5} fz="0.6em" opacity="0.6">
                        {description}
                    </Text>
                </Text>
            </Stack>
        </Combobox.Option>
    );
}

interface TagBadageProps {
    subjectName: string;

    name: string;

    prefix?: string;
}

function TagBadage(props: TagBadageProps) {
    const { subjectName, name, prefix } = props;
    return (
        <Stack gap={0} p={0}>
            <Text component="span" h="0.65em" fz="0.65em" opacity="0.5">{subjectName}</Text>
            <Text component="span" h="1.1em">{name}</Text>
        </Stack>
    );
}

const OPERATION_ITEM: { [key: string]: InputOptionType } = [
    { name: '+', description: 'include tag' },
    { name: '-', description: 'exclude tag' },
    { name: '[', description: 'bracket' },
    { name: ']', description: 'bracket' },
].reduce<{ [key: string]: InputOptionType }>((prev, current) => ({
    ...prev,
    [current.name]: {
        groupName:   'Operator',
        key:         current.name,
        itemID:      current.name,
        name:        current.name,
        value:       current.name,
        description: current.description,
        isOperator:  true,
    },
}), {});

enum InputStatus {
    Initial,
    PrefixOperator, // -, +
    TagName, // tag, left bracket
    LeftBracket,
}

type InputStatusCache = {
    status: InputStatus;
    text: string;
    display: ReactNode[];
}

type InputStatusMechine = {
    name: InputStatus;

    options: ('tag' | '+' | '-' | '[' | ']')[];

    action: (val: string, option: ComboboxOptionProps) => InputStatus;
}

const useComplexSearch = (tags: TagResDto[], searchText: string) => {
    const [currentInputStatus, setCurrentInputStatus] = useState<InputStatus>(InputStatus.Initial);
    const statusStackRef = useRef<InputStatusCache[]>([]);

    const [staticText, setStaticText] = useState<string>('');
    const [displayNode, setDisplayNode] = useState<ReactNode[]>([]);

    const tagOptionProps: InputOptionType[] = useMemo(() => (
        tags.map<InputOptionType>((item) => ({
            key:         item.id,
            name:        item.name,
            groupName:   item.subject_name,
            description: item.description,
            value:       `${item.subject_name}:${item.name}`,
        }))
    ), [tags]);

    const statusMechine: Map<InputStatus, InputStatusMechine> = useMemo(() => {
        const map = new Map<InputStatus, InputStatusMechine>();
        const status: InputStatusMechine[] = [
            {
                name:    InputStatus.Initial,
                options: ['+', '-'],
                action:  (val, option) => {
                    setStaticText((prev) => (`${prev} ${val}`));
                    setDisplayNode((prev) => [...prev, val]);
                    return InputStatus.PrefixOperator;
                },
            },
            {
                name:    InputStatus.PrefixOperator,
                options: ['tag', '['],
                action:  (val, option) => {
                    setStaticText((prev) => (prev  + ' ' + val));
                    if (val === '[') {
                        setDisplayNode((prev) => [...prev, val]);
                        return InputStatus.LeftBracket;
                    }
                    setDisplayNode((prev) => [...prev, <TagBadage subjectName={option['data-groupName']} name={option['data-name']} />]);
                    return InputStatus.Initial;
                },
            },
            {
                name:    InputStatus.TagName,
                options: ['tag', ']'],
                action:  (val, option) => {
                    // eslint-disable-next-line prefer-template
                    setStaticText((prev) => (prev + ' ' + val));
                    if (val === ']') {
                        setDisplayNode((prev) => [...prev, ' ' + val]);
                        return InputStatus.Initial;
                    }
                    setDisplayNode((prev) => [...prev, <TagBadage subjectName={option['data-groupName']} name={option['data-name']} />]);
                    return InputStatus.TagName;
                },
            },
            {
                name:    InputStatus.LeftBracket,
                options: ['tag'],
                action:  (val, option) => {
                    // eslint-disable-next-line prefer-template
                    setStaticText((prev) => (prev + ' ' + val));
                    if (val === ']') {
                        setDisplayNode((prev) => [...prev, ' ' + val]);
                        return InputStatus.Initial;
                    }
                    setDisplayNode((prev) => [...prev, <TagBadage subjectName={option['data-groupName']} name={option['data-name']} />]);
                    return InputStatus.TagName;
                },
            },
        ];
        status.forEach((val) => map.set(val.name, val));
        return map;
    }, []);

    const selectableOptions = useMemo(() => {
        const mechine = statusMechine.get(currentInputStatus)!;
        return mechine.options
            .reduce<InputOptionType[]>((prev, val) => (val === 'tag' ? [...prev, ...tagOptionProps] : [...prev, OPERATION_ITEM[val]]), [])
            .filter((item) => item.value.toLowerCase().includes(searchText.toLowerCase().trim()))
            .map((item, index) => (
                <InputOption
                    // eslint-disable-next-line react/jsx-props-no-spreading
                    {...item}
                    selected={index === 0}
                />
            ));
    }, [statusMechine, currentInputStatus, tagOptionProps, searchText]);

    const backspaceSearch: () => InputStatusCache = useCallback(() => {
        if (statusStackRef.current.length <= 0) {
            setCurrentInputStatus(InputStatus.Initial);
            setStaticText('');
            return {
                status:  InputStatus.Initial,
                display: [],
                text:    '',
            };
        }
        const preStatus = statusStackRef.current.pop();
        setCurrentInputStatus(preStatus!.status);
        setDisplayNode(preStatus!.display);
        setStaticText(preStatus!.text);
        return preStatus!;
    }, []);

    const forwardSearch = useCallback((val: string, comboxOptionProps: ComboboxOptionProps) => {
        statusStackRef.current.push({
            status:  currentInputStatus,
            text:    staticText,
            display: displayNode,
        });
        const nextStatus = statusMechine.get(currentInputStatus)!.action(val, comboxOptionProps);
        setCurrentInputStatus(nextStatus);
    }, [statusMechine, currentInputStatus, displayNode, staticText]);

    return {
        options:       selectableOptions,
        displayNode,
        displayText:   staticText,
        currentStatus: currentInputStatus,
        backspaceSearch,
        forwardSearch,
    };
};

export interface ComplexSearchInputProps {
    tags: TagResDto[]
}

export function ComplexSearchInput(props: ComplexSearchInputProps) {
    const { tags } = props;
    const combobox = useCombobox({ onDropdownClose: () => combobox.resetSelectedOption() });
    const [searchText, setSearchText] = useState<string>('');
    const { options, displayNode, displayText, backspaceSearch, forwardSearch } = useComplexSearch(tags, searchText);

    const handleOptionSubmit = useCallback((val: string, comboxOptionProps: ComboboxOptionProps) => {
        forwardSearch(val, comboxOptionProps);
        combobox.selectFirstOption();
        setSearchText('');
    }, [combobox, forwardSearch]);

    return (
        <Combobox
            position="bottom"
            store={combobox}
            onOptionSubmit={handleOptionSubmit}
        >
            <Combobox.Target>
                <Group classNames={{ root: classes.searchRoot }}>
                    <Flex gap="sm">
                        {displayText}
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
                                backspaceSearch();
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
