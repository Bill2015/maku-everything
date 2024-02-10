import { CSSProperties, Stack, StackProps, Text } from '@mantine/core';

import classes from './TagTypography.module.scss';

export interface TagTypographyProps extends StackProps {
    name: string;

    description?: string;

    subjectName: string;

    fontSize?: number;

    styles?: Partial<Record<'root' | 'subject' | 'main' | 'description', CSSProperties>>
}

export function TagTypography(props: TagTypographyProps) {
    const { description, name, subjectName, fontSize = 1, styles, ...stackProps } = props;

    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <Stack gap={0} {...stackProps} styles={{ root: styles?.root }}>
            <Text
                component="span"
                h="0.75em"
                className={classes.subject}
                fz={`${fontSize - 0.3}rem`}
                style={styles?.subject}
            >
                {subjectName}
            </Text>
            <Text
                component="span"
                h="1.1em"
                fw="bold"
                fz={`${fontSize}rem`}
                style={styles?.main}
            >
                {name}
                {
                    description && (
                        <Text
                            component="span"
                            className={classes.description}
                            fz={`${fontSize - 0.4}rem`}
                            style={styles?.description}
                        >
                            {description}
                        </Text>
                    )
                }
            </Text>
        </Stack>
    );
}
