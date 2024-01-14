import { useCallback, useMemo, useRef, useState } from 'react';
import { Combobox, ComboboxOptionProps, Group, Input, Stack, Text, useCombobox } from '@mantine/core';
import { TagResDto } from '@api/tag';

export interface InputOptionProps extends ComboboxOptionProps {
    description: string;

    name: string;

    title: string;
}

export function InputOption(props: InputOptionProps) {
    const { description, name, title, ...optionProps } = props;

    return (
        <Combobox.Option
            aria-description={description}
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...optionProps}
        >
            <Stack gap={0} p={0}>
                <Text component="span" h="0.75em" fz="0.75em" opacity="0.5">{title}</Text>
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

const OPERATION_ITEM: { [key: string]: JSX.Element } = [
    { name: '+', description: 'include tag' },
    { name: '-', description: 'exclude tag' },
    { name: '(', description: 'bracket' },
    { name: ')', description: 'bracket' },
].reduce((prev, current) => ({
    ...prev,
    [current.name]: <InputOption
        title="Operator"
        itemID={current.name}
        name={current.name}
        value={current.name}
        description={current.description}
    />,
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
}

// Initial =| PrefixOperator

// PrefixOperator =| TagName
//                 | Left Bracket

// TagName =| Initial
//          | TagName
//          | Right Bracket

// LeftBracket =| TagName

export interface ComplexSearchInputProps {
    tags: TagResDto[]
}

export function ComplexSearchInput(props: ComplexSearchInputProps) {
    const { tags } = props;
    const [currentInputStatus, setCurrentInputStatus] = useState<InputStatus>(InputStatus.Initial);

    const statusStackRef = useRef<InputStatusCache[]>([]);

    const [staticText, setStaticText] = useState<string>('');
    const [searchText, setSearchText] = useState<string>('');
    const combobox = useCombobox({ onDropdownClose: () => combobox.resetSelectedOption() });

    const totalOptions: JSX.Element[] = [];

    const filteredTagOptions = useMemo(() => (
        tags
            .filter((item) => `${item.subject_name}:${item.name}`.toLowerCase().includes(searchText.toLowerCase().trim()))
            .map((item) => (
                <InputOption
                    itemID={item.id}
                    title={item.subject_name}
                    name={item.name}
                    description={item.description}
                    value={`${item.subject_name}:${item.name}`}
                />
            ))
    ), [tags, searchText]);

    if (currentInputStatus === InputStatus.Initial) {
        totalOptions.push(OPERATION_ITEM['+']);
        totalOptions.push(OPERATION_ITEM['-']);
    }

    if (currentInputStatus === InputStatus.PrefixOperator) {
        totalOptions.push(...filteredTagOptions);
        totalOptions.push(OPERATION_ITEM['(']);
    }

    if (currentInputStatus === InputStatus.TagName) {
        totalOptions.push(...filteredTagOptions);
        totalOptions.push(OPERATION_ITEM[')']);
    }

    if (currentInputStatus === InputStatus.LeftBracket) {
        totalOptions.push(...filteredTagOptions);
    }

    const handleOptionSubmit = useCallback((val: string, options: ComboboxOptionProps) => {
        statusStackRef.current.push({
            status: currentInputStatus,
            text:   staticText,
        });
        if (currentInputStatus === InputStatus.Initial) {
            setStaticText((prev) => (`${prev} ${val}`));
            setCurrentInputStatus(InputStatus.PrefixOperator);
        }
        if (currentInputStatus === InputStatus.PrefixOperator) {
            if (val === '(') {
                setStaticText((prev) => (prev + val));
                setCurrentInputStatus(InputStatus.LeftBracket);
            }
            else {
                // eslint-disable-next-line prefer-template
                setStaticText((prev) => (prev + val));
                setCurrentInputStatus(InputStatus.Initial);
            }
        }

        if (currentInputStatus === InputStatus.LeftBracket || currentInputStatus === InputStatus.TagName) {
            if (val === ')') {
                setStaticText((prev) => (`${prev} ${val}`));
                setCurrentInputStatus(InputStatus.Initial);
            }
            else {
                // eslint-disable-next-line prefer-template
                setStaticText((prev) => (prev + ' ' + val));
                setCurrentInputStatus(InputStatus.TagName);
            }
        }
        setSearchText('');
    }, [staticText, currentInputStatus]);

    return (
        <Combobox
            position="bottom"
            store={combobox}
            onOptionSubmit={handleOptionSubmit}
        >
            <Combobox.Target>
                <Group>
                    <Text>{staticText}</Text>
                    <Input
                        value={searchText}
                        placeholder="search here..."
                        onChange={(e) => {
                            setSearchText(e.currentTarget.value);
                        }}
                        onKeyDown={(e) => {
                            if (e.key === 'Backspace' && searchText === '') {
                                if (statusStackRef.current.length <= 0) {
                                    setCurrentInputStatus(InputStatus.Initial);
                                    setStaticText('');
                                }
                                else {
                                    const preStatus = statusStackRef.current.pop();
                                    setCurrentInputStatus(preStatus!.status);
                                    setStaticText(preStatus!.text);
                                }
                            }
                        }}
                        onClick={() => combobox.toggleDropdown()}
                        style={{ flexGrow: 1 }}
                    />
                </Group>
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Options mah="50dvh" style={{ overflowY: 'auto' }}>
                    {
                        (totalOptions.length > 0)
                            ? totalOptions
                            : <Combobox.Empty>Nothing found</Combobox.Empty>
                    }
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
}

// +AI => 只顯示包含 AI 標籤的資源
// -AI => 不顯示包含 AI 標籤的資源
// +AI +Python => 顯示包含 AI 與 Python 標籤的資源
// +(AI | Python) +Javascript => 顯示一定要有 Javascript 但是可能包含 AI 或 Python 標籤的資源
// -(AI | Python) +Javascript === -AI -Python +Javascript;
// -(AI & Python) +Javascript => 顯示一定要有 Javascript 但是不能同時包含 AI 與 Python 標籤的資源
// +(AI & Python) +Javascript === +AI +Python +Javascript;
