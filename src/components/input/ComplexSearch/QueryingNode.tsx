import { Stack, Text } from '@mantine/core';
import classes from './QueryingNode.module.scss';

export interface QueryingNodeProps {
    type: 'tag' | 'string';

    groupName?: string;

    label: string;

    prefix?: string;
}

export function QueryingNode(props: QueryingNodeProps) {
    const { type, groupName, label, prefix = '' } = props;
    return (
        <Stack gap={0} p={0}>
            <Text component="span" h="0.65em" className={classes.group}>{groupName}</Text>
            <Text component="span" h="1.1em">
                {prefix}
                {label}
            </Text>
        </Stack>
    );
}
