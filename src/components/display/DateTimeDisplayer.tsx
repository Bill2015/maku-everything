/* eslint-disable react/jsx-props-no-spreading */
import { useTranslation } from 'react-i18next';
import { Box, Text, TextProps } from '@mantine/core';
import { DateTimeUnit, formatDateTime, getDateTimeInterval } from '@utils/date';
import { normalizeKey } from '@modules/i18next';

export interface DateTimeDisplayerProps extends TextProps {
    date: string;

    label: string;
}

const DATETIME_DISPLAYER_MAPPER: Record<DateTimeUnit, string> = {
    seconds: 'seconds_ago',
    minutes: 'minutes_ago',
    hours:   'hours_ago',
    days:    'days_ago',
    weeks:   'weeks_ago',
    months:  'months_ago',
    years:   'years_ago',
};

export function DateTimeDisplayer(props: DateTimeDisplayerProps) {
    const { date, label, ...textProps } = props;
    const { t } = useTranslation('common', { keyPrefix: 'Display.DateTimeDisplayer' });

    const result = getDateTimeInterval(date);

    return (
        <Box>
            <Text fz="0.75rem" opacity="0.75">
                {label}
            </Text>
            <Text style={{ lineHeight: 0.7 }} {...textProps}>
                {
                    result
                        ? `${result.value} ${t(normalizeKey(DATETIME_DISPLAYER_MAPPER[result.stamp]))}`
                        : t('just_now')
                }
                <Text component="span" pl={5} fz={10} opacity="0.5" style={{ lineHeight: 0.5 }}>
                    {`(${formatDateTime(date)})`}
                </Text>
            </Text>
        </Box>

    );
}
