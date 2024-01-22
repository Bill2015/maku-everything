/* eslint-disable react/jsx-props-no-spreading */
import React, { useCallback, useState } from 'react';
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

    const handleOptionSubmit = useCallback((optionVal: string, comboxOptionProps: ComboboxOptionProps) => {
        forwardInputSearch(searchText, optionVal, comboxOptionProps);
        combobox.resetSelectedOption();
        setSearchText('');
    }, [combobox, searchText, forwardInputSearch]);

    const handleKeyDown = useCallback((e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Backspace' && searchText === '') {
            const history = backspaceInputSearch();
            setSearchText(history.inputs.at(-1)!);
            e.preventDefault();
            return;
        }
        if (e.key === 'Enter' && searchText === '' && combobox.getSelectedOptionIndex() < 0) {
            onSubmitSearch(rawText);
            combobox.resetSelectedOption();
            combobox.closeDropdown();
            return;
        }
        if (e.key === 'Enter') {
            const val = e.currentTarget.value;
            if (forwardInputSearch(val, val, null)) {
                setSearchText('');
                return;
            }
        }
        combobox.openDropdown();
    }, [backspaceInputSearch, forwardInputSearch, onSubmitSearch, combobox, rawText, searchText]);

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
                            onKeyDown={handleKeyDown}
                            onFocus={() => {
                                combobox.openDropdown();
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
