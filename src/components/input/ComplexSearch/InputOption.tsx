import { Combobox, ComboboxOptionProps, Stack, Text } from '@mantine/core';
import { InputSymbol } from './enums';
import classses from './InputOption.module.scss';

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

    suffix?: string;
}

export interface InputOptionProps extends InputOptionType, Omit<ComboboxOptionProps, 'key'> { }

/**
 * Display the input options */
export function InputOption(props: InputOptionProps) {
    const { description, name, groupName, suffix, ...optionProps } = props;

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
                        {description}
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
    { name: InputSymbol.Include, description: 'include tag' },
    { name: InputSymbol.Exclude, description: 'exclude tag' },
    { name: InputSymbol.LeftBracket, description: 'bracket' },
    { name: InputSymbol.RightBracket, description: 'bracket' },
].reduce<Operators>((prev, current) => ({
    ...prev,
    [current.name]: {
        groupName:   'Operator',
        key:         current.name,
        itemID:      current.name,
        name:        current.name,
        value:       current.name,
        description: current.description,
    },
}), {});
