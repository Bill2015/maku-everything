import { ElementProps, Text, TextProps } from '@mantine/core';

export interface SubTitleProps extends TextProps, ElementProps<'div', keyof TextProps> { }

export function SubTitle(props: SubTitleProps) {
    const { children } = props;
    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <Text c="dimmed" fw="bolder" fz="sm" {...props}>{children}</Text>
    );
}
