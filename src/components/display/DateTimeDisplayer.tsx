/* eslint-disable react/jsx-props-no-spreading */
import { Box, Text, TextProps } from '@mantine/core';
import { DateTimeUnit, formatDateTime, getDateTimeInterval } from '@utils/date';

export interface DateTimeDisplayerProps extends TextProps {
    date: string;

    label: string;
}

const DATETIME_DISPLAYER_MAPPER: { [key in DateTimeUnit]: string } = {
    minutes: 'minutes ago',
    hours:   'hours ago',
    days:    'days ago',
    months:  'months ago',
    years:   'years ago',
    weeks:   'weeks ago',
};

export function DateTimeDisplayer(props: DateTimeDisplayerProps) {
    const { date, label, ...textProps } = props;

    const result = getDateTimeInterval(date);

    if (!result) {
        return <Text>Time travel is real!</Text>;
    }

    return (
        <Box>
            <Text fz="0.75rem" opacity="0.75">
                {label}
            </Text>
            <Text style={{ lineHeight: 0.7 }} {...textProps}>
                {`${result.value} ${DATETIME_DISPLAYER_MAPPER[result.stamp]}`}
                <Text component="span" pl={5} fz={10} opacity="0.5" style={{ lineHeight: 0.5 }}>
                    {`(${formatDateTime(date)})`}
                </Text>
            </Text>
        </Box>

    );
}
