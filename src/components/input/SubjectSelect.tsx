import { MouseEvent, Ref, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Combobox, ComboboxItem, Group, InputBase, Text, useCombobox } from '@mantine/core';
import { SubjectResDto } from '@api/subject';

import classes from './SubjectSelect.module.scss';

export interface SubjectSelectItemProps extends ComboboxItem {
    id: string;

    description: string;
}

export type SubjectSelectItemData = Omit<SubjectSelectItemProps, 'label' | 'disable'>;

function SubjectSelectItem(props: SubjectSelectItemProps) {
    const { description, id, label } = props;

    return (
        <Group id={id}>
            <Text>{label}</Text>
            <Text size="xs" color="dimmed">{description}</Text>
        </Group>
    );
}

export interface SubjectSelectProps {
    inputRef?: Ref<HTMLInputElement>;

    subjects: SubjectResDto[];

    hidden?: boolean;

    onItemSelect: (data: Omit<SubjectSelectItemProps, 'label'|'disable'>) => void;

    value?: string | null;

    onClickResult?: (e: MouseEvent) => void;
}

export function SubjectSelect(props: SubjectSelectProps) {
    const { inputRef, value, hidden, subjects, onItemSelect, onClickResult } = props;
    const { t } = useTranslation('common', { keyPrefix: 'Input.SubjectSelect' });

    const [search, setSearch] = useState('');

    const combobox = useCombobox({ onDropdownClose: () => combobox.resetSelectedOption() });

    const options = subjects
        .filter((item) => item.name.toLowerCase().includes(search.toLowerCase().trim()))
        .map((item) => (
            <Combobox.Option itemID={item.id} value={item.name} key={item.name} aria-description={item.description}>
                <SubjectSelectItem description={item.description} id={item.id} value={item.name} label={item.name} />
            </Combobox.Option>
        ));

    if (value) {
        return <Text className={classes.resultBox} onClick={onClickResult}>{value}</Text>;
    }

    return (
        <Combobox
            store={combobox}
            position="bottom"
            withinPortal={false}
            onOptionSubmit={(val, option) => {
                const data = {
                    id:          option.itemID!,
                    value:       val,
                    description: option['aria-description']!,
                };
                onItemSelect(data);
                setSearch(val);
                combobox.closeDropdown();
            }}
        >
            <Combobox.Target>
                <InputBase
                    autoFocus
                    pointer
                    placeholder={t('placeholder')}
                    rightSectionPointerEvents="none"
                    display={hidden ? 'none' : 'initial'}
                    ref={inputRef}
                    value={search}
                    onChange={(event) => {
                        combobox.openDropdown();
                        combobox.updateSelectedOptionIndex();
                        setSearch(event.currentTarget.value);
                    }}
                    onBlur={() => {
                        combobox.closeDropdown();
                        setSearch('');
                    }}
                    onClick={() => combobox.toggleDropdown()}
                />
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Options>
                    {
                        (options.length > 0)
                            ? options
                            : <Combobox.Empty>{t('not_found')}</Combobox.Empty>
                    }
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
}
