import { Stack, StackProps, Text } from '@mantine/core';

import classes from './TagTypography.module.scss';

export interface TagTypographyProps extends StackProps {
    name: string;

    description?: string;

    subjectName: string;
}

export function TagTypography(props: TagTypographyProps) {
    const { description, name, subjectName, ...stackProps } = props;

    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <Stack gap={0} {...stackProps}>
            <Text component="span" h="0.75em" className={classes.subject}>
                {subjectName}
            </Text>
            <Text component="span" h="1.1em" fw="bold">
                {name}
                {
                    description && (
                        <Text component="span" className={classes.description}>
                            {description}
                        </Text>
                    )
                }
            </Text>
        </Stack>
    );
}
