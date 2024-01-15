import { Combobox, ComboboxOptionProps, Stack, Text } from '@mantine/core';
import classses from './InputOption.module.scss';

export type InputOptionType = {
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
            className={classses.root}
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...optionProps}
        >
            <Stack gap={0} p={0}>
                <Text component="span" h="0.75em" className={classses.group}>
                    {groupName}
                </Text>
                <Text component="span" h="1.1em">
                    {name}
                    <Text component="span" className={classses.description}>
                        {description}
                    </Text>
                </Text>
            </Stack>
        </Combobox.Option>
    );
}

type Operators = { [key: string]: InputOptionType };

InputOption.Operators = [
    { name: '+', description: 'include tag' },
    { name: '-', description: 'exclude tag' },
    { name: '[', description: 'bracket' },
    { name: ']', description: 'bracket' },
].reduce<Operators>((prev, current) => ({
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
