/* eslint-disable react/jsx-props-no-spreading */
import { forwardRef, useImperativeHandle, useMemo, useRef, useState } from 'react';
import { Combobox, ComboboxOptionProps, ElementProps, Flex, Input, InputProps, useCombobox } from '@mantine/core';
import { IoIosArrowDown } from 'react-icons/io';
import { TagTypography } from '@components/display';
import { TagResDto } from '@api/tag';

import classes from './TagComboSelect.module.scss';

type CustomOptionProps = {
    'data-id': string,
    'data-name': string,
    'data-description': string,
    'data-subjectname': string,
}

export type TagSelectOptionValue = {
    id: string;

    value: string;

    name: string;

    description?: string;

    subjectName: string;
}

export interface TagSelectOptionProps extends TagSelectOptionValue,
    Omit<ComboboxOptionProps, 'id'>,
    ElementProps<'option', keyof ComboboxOptionProps> {
}

function TagOption(props: TagSelectOptionProps) {
    const { id, description, name, subjectName, value, ...optionProps } = props;

    return (
        <Combobox.Option
            data-id={id}
            data-subjectname={subjectName}
            data-description={description}
            data-name={name}
            value={value}
            {...optionProps}
        >
            <TagTypography
                name={name}
                description={description}
                subjectName={subjectName}
            />
        </Combobox.Option>
    );
}

export interface TagComboSelectProps {
    data: TagSelectOptionValue[];

    defaultValue?: TagSelectOptionValue | undefined;

    onSubmitOptions: (option: TagSelectOptionValue | null) => void;

    inputProps?: InputProps;
}

export interface TagComboSelectRef {
    getInputRef: () => HTMLInputElement | null;

    clearInput: () => void;
}

export const TagComboSelect = forwardRef<TagComboSelectRef, TagComboSelectProps>((props, ref) => {
    const { data, defaultValue, onSubmitOptions, inputProps } = props;
    const inputRef = useRef<HTMLInputElement>(null);
    const combobox = useCombobox({ onDropdownClose: () => combobox.resetSelectedOption() });
    const [searchText, setSearchText] = useState<string>('');
    const [selectedValue, setSelectedValue] = useState<TagSelectOptionValue | null>(null);

    const options = useMemo(() => data.filter((val) => val.value.includes(searchText.toLowerCase())), [data, searchText]);

    const hanldeOptionSubmit = (val: string, option: ComboboxOptionProps & Partial<CustomOptionProps>) => {
        const result: TagSelectOptionValue = {
            value:       option.value,
            id:          option['data-id']!,
            name:        option['data-name']!,
            description: option['data-description']!,
            subjectName: option['data-subjectname']!,
        };
        setSelectedValue(result);
        onSubmitOptions(result);
    };

    useImperativeHandle(ref, () => ({
        getInputRef: () => inputRef.current,
        clearInput:  () => {
            setSelectedValue(null);
            setSearchText('');
        },
    }));

    const showedValue = selectedValue || defaultValue;

    return (
        <Combobox
            store={combobox}
            onOptionSubmit={hanldeOptionSubmit}
        >
            <Combobox.Target>
                <Flex className={classes.inputGroup}>
                    {showedValue && (
                        <TagTypography
                            name={showedValue.name}
                            description=""
                            subjectName={showedValue.subjectName}
                            pl="xs"
                        />
                    )}
                    <Input
                        ref={inputRef}
                        flex={1}
                        classNames={classes}
                        rightSection={<IoIosArrowDown />}
                        {...inputProps}
                        value={searchText}
                        onChange={(e) => {
                            setSearchText(e.currentTarget.value);
                        }}
                        onKeyDown={(e) => {
                            if (e.key === 'Backspace' && !e.currentTarget.value) {
                                setSelectedValue(null);
                                onSubmitOptions(null);
                            }
                        }}
                        onFocus={() => {
                            combobox.openDropdown();
                            combobox.resetSelectedOption();
                        }}
                        onBlur={() => {
                            combobox.closeDropdown();
                        }}
                    />
                </Flex>

            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Options mah="40dvh" style={{ overflowY: 'auto' }}>
                    {
                        (options.length > 0)
                            ? options.map((val) => <TagOption key={val.id} {...val} />)
                            : <Combobox.Empty>not_found</Combobox.Empty>
                    }
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
});

export function useTagComboSelectValue(values: TagResDto[]) {
    return useMemo(() => values.map<TagSelectOptionValue>((val) => ({
        id:          val.id,
        value:       `${val.subject_name}:${val.name}`.toLowerCase(),
        description: val.description,
        name:        val.name,
        subjectName: val.subject_name,
    })), [values]);
}
