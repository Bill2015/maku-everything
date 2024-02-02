import { useTranslation } from 'react-i18next';
import { Combobox, ComboboxOptionProps, Stack, Text } from '@mantine/core';
import { System } from '@declares/variables';
import { normalizeKey } from '@modules/i18next';

import { InputSymbol } from './enums';
import classses from './InputOption.module.scss';

export type AttributeType = 'none' | 'text' | 'option-text' | 'range-number' | 'range-date';

export type ComboboxOptionWithDataProps = ComboboxOptionProps & {
    'data-groupname'?: string;

    'data-name'?: string;
}

export type InputOptionType = {
    key: string;

    description: string;

    name: string;

    value: string;

    groupName: string;

    attributeType: AttributeType;

    suffix?: string;

    i18nextDescription?: boolean;
}

export interface InputOptionProps extends InputOptionType, Omit<ComboboxOptionProps, 'key'> { }

/**
 * Display the input options */
export function InputOption(props: InputOptionProps) {
    const { description, name, groupName, suffix, i18nextDescription = false, attributeType: _, ...optionProps } = props;
    const { t } = useTranslation('common', { keyPrefix: 'Input.ComplexSearchInput.Options' });

    return (
        <Combobox.Option
            aria-description={description}
            data-groupname={groupName}
            data-name={name}
            className={classses.root}
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...optionProps}
        >
            <Stack gap={0} p={0}>
                <Text component="span" h="0.75em" className={classses.group}>
                    {groupName}
                </Text>
                <Text component="span" h="1.1em" fw="bold">
                    {name}
                    <Text component="span" className={classses.suffix}>{suffix}</Text>
                    <Text component="span" className={classses.description}>
                        {i18nextDescription ? t(normalizeKey(description)) : description}
                    </Text>
                </Text>
            </Stack>
        </Combobox.Option>
    );
}

type Operators = { [key in InputSymbol]?: InputOptionType };

/**
 * Selectable Operators */
InputOption.Operators = [
    { name: InputSymbol.Include, description: 'op_include' },
    { name: InputSymbol.Exclude, description: 'op_exclude' },
    { name: InputSymbol.LeftGroupBracket, description: 'op_group_bracket' },
    { name: InputSymbol.RightGroupBracket, description: 'op_group_bracket' },
    { name: InputSymbol.LeftAttrBracket, description: 'op_attr_bracket' },
    { name: InputSymbol.RightAttrBracket, description: 'op_attr_bracket' },
].reduce<Operators>((prev, current) => ({
    ...prev,
    [current.name]: {
        groupName:          'Operator',
        key:                current.name,
        itemID:             current.name,
        name:               current.name,
        value:              current.name,
        description:        current.description,
        i18nextDescription: true,
        attributeType:      'none',
    } as InputOptionType,
}), {});

/**
 * Functional tag */
InputOption.FunctionalTags = ([
    {
        name: 'url', description: 'func_url', attributeType: 'option-text',
    },
    {
        name: 'file', description: 'func_file', attributeType: 'option-text',
    },
    {
        name: 'filext', description: 'func_filext', attributeType: 'text',
    },
    {
        name: 'name', description: 'func_name', attributeType: 'text',
    },
    {
        name: 'tagnum', description: 'func_tagnum', attributeType: 'range-number',
    },
    {
        name: 'created', description: 'func_created', attributeType: 'range-date',
    },
    {
        name: 'updated', description: 'func_updated', attributeType: 'range-date',
    },
] as { name: string, description: string, attributeType: AttributeType }[])
    .map<InputOptionType>((item) => ({
        groupName:          System.Namesapce,
        key:                item.name,
        itemID:             item.name,
        name:               item.name,
        value:              `${System.Namesapce}:${item.name}`,
        description:        item.description,
        attributeType:      item.attributeType,
        i18nextDescription: true,
    }));
