/* eslint-disable react/jsx-props-no-spreading */
import { useCallback, useState } from 'react';
import { ActionIcon, Combobox, ComboboxOptionProps, Flex, Group, Input, useCombobox } from '@mantine/core';
import { randomId } from '@mantine/hooks';
import { FaSearch } from 'react-icons/fa';
import { ImCross } from 'react-icons/im';

import { TagResDto } from '@api/tag';
import { useComplexSearch } from './hooks';

import classes from './ComplexSearchInput.module.scss';
import { QueryingNode } from './QueryingNode';
import { InputOption } from './InputOption';

export interface ComplexSearchInputProps {
    tags: TagResDto[];

    onSubmitSearch: (searchText: string) => void;

    onClearSearch: () => void;
}

export function ComplexSearchInput(props: ComplexSearchInputProps) {
    const { tags, onSubmitSearch, onClearSearch } = props;
    const combobox = useCombobox({ onDropdownClose: () => combobox.resetSelectedOption() });
    const [searchText, setSearchText] = useState<string>('');
    const { options, displayNode, rawText, backspaceInputSearch, forwardInputSearch, clearSearch } = useComplexSearch(tags, searchText);

    const handleOptionSubmit = useCallback((val: string, comboxOptionProps: ComboboxOptionProps) => {
        forwardInputSearch(val, comboxOptionProps);
        combobox.resetSelectedOption();
        setSearchText('');
    }, [combobox, forwardInputSearch]);

    return (
        <Combobox
            position="bottom"
            store={combobox}
            onOptionSubmit={handleOptionSubmit}
        >
            <Combobox.Target>
                <Flex classNames={{ root: classes.searchRoot }}>
                    <Group gap="0.6rem" className={classes.displayQuery}>
                        {displayNode.map((val) => <QueryingNode key={randomId()} {...val} />) }
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
                            onClick={() => {
                                combobox.toggleDropdown();
                                combobox.resetSelectedOption();
                            }}
                        />
                    </Group>
                    <ActionIcon
                        className={classes.searchBtn}
                        onClick={() => {
                            onSubmitSearch(rawText);
                        }}
                    >
                        <FaSearch />
                    </ActionIcon>
                    {
                        rawText && (
                            <ActionIcon
                                className={classes.clearBtn}
                                onClick={() => {
                                    clearSearch();
                                    setSearchText('');
                                    onClearSearch();
                                }}
                            >
                                <ImCross />
                            </ActionIcon>
                        )
                    }
                </Flex>
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Options mah="50dvh" style={{ overflowY: 'auto' }}>
                    {
                        (options.length > 0)
                            ? options.map((val) => <InputOption {...val} />)
                            : <Combobox.Empty>Nothing found</Combobox.Empty>
                    }
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
}
