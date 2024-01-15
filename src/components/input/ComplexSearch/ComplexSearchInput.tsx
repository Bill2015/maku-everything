/* eslint-disable react/jsx-props-no-spreading */
import { useCallback, useState } from 'react';
import { ActionIcon, Combobox, ComboboxOptionProps, Flex, Group, Input, useCombobox } from '@mantine/core';
import { FaSearch } from 'react-icons/fa';

import { TagResDto } from '@api/tag';
import { useComplexSearch } from './hooks';

import classes from './ComplexSearchInput.module.scss';
import { QueryingNode } from './QueryingNode';
import { InputOption } from './InputOption';

export interface ComplexSearchInputProps {
    tags: TagResDto[]
}

export function ComplexSearchInput(props: ComplexSearchInputProps) {
    const { tags } = props;
    const combobox = useCombobox({ onDropdownClose: () => combobox.resetSelectedOption() });
    const [searchText, setSearchText] = useState<string>('');
    const { options, displayNode, rawText, backspaceInputSearch, forwardInputSearch } = useComplexSearch(tags, searchText);

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
                <Flex classNames={{ root: classes.searchRoot }}>
                    <Group gap="sm" className={classes.displayQuery}>
                        {displayNode.map((val) => <QueryingNode key={val.label} {...val} />) }
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
                                combobox.selectFirstOption();
                            }}
                        />
                    </Group>
                    <ActionIcon className={classes.searchBtn}>
                        <FaSearch />
                    </ActionIcon>
                </Flex>
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Options mah="50dvh" style={{ overflowY: 'auto' }}>
                    {
                        (options.length > 0)
                            ? options.map((val, index) => <InputOption {...val} selected={index === 0} />)
                            : <Combobox.Empty>Nothing found</Combobox.Empty>
                    }
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
}
