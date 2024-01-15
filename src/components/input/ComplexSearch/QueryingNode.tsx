import { Stack, Text } from '@mantine/core';
import classes from './QueryingNode.module.scss';

export interface QueryingNodeProps {
    /** `Tag` or normal string \
     *  If it's a tag it will clickable link */
    type: 'tag' | 'string';

    /**
     * Group name of options */
    groupName?: string;

    /**
     * Disaply label */
    label: string;

    /**
     * That can display prefix operator in front of tag name */
    prefix?: string;
}

/**
 * Display `Query String` as a component */
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
