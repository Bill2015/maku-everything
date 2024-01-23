import { Group, Stack, Text } from '@mantine/core';
import classes from './QueryingNode.module.scss';

export interface QueryingNodeProps {
    /** `Tag` or normal string \
     *  If it's a tag it will clickable link */
    type: 'tag' | 'operator' | 'attribute' | 'display-only';

    /**
     * Group name of options */
    groupName?: string;

    /**
     * Disaply label */
    label: string;

    /**
     * That can display prefix operator in front of tag name */
    prefix?: string;

    /**
     * That can display suffix attribute after the tag name */
    suffix?: string;
}

/**
 * Display `Query String` as a component */
export function QueryingNode(props: QueryingNodeProps) {
    const { type, groupName, label, prefix = '', suffix = '' } = props;

    if (type === 'tag') {
        return (
            <Group gap="0.1rem">
                <Text component="span" h="1.1em" fz="xl" fw="bolder">{prefix}</Text>
                <Stack gap={0} p={0}>
                    <Text component="span" h="0.65em" className={classes.group}>{groupName}</Text>
                    <Text component="span" h="1.1em">
                        {label}
                        <Text component="span" h="0.54" fz="1rem" pl="0.3rem" c="blue">{suffix}</Text>
                    </Text>
                </Stack>
            </Group>
        );
    }

    if (type === 'display-only') {
        return (
            <Text component="span" h="1.1rem" fz="sm" c="indigo">
                {label}
            </Text>
        );
    }

    return (
        <Text component="span" h="1.1rem" fz="xl" c="indigo">
            {prefix}
            {label}
        </Text>
    );
}
