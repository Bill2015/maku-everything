import {
    format,
    parseISO,
    differenceInYears,
    differenceInMonths,
    differenceInWeeks,
    differenceInDays,
    differenceInHours,
    differenceInMinutes,
    differenceInSeconds,
} from 'date-fns';

export type DateTimeUnit = 'years' | 'months' | 'weeks' | 'days' | 'hours' | 'minutes' | 'seconds';

export type DateTimeInterval = { value: number, stamp: DateTimeUnit };

const DIFFS: { diffn: typeof differenceInYears, stamp: DateTimeUnit }[] = [
    { diffn: differenceInYears, stamp: 'years' },
    { diffn: differenceInMonths, stamp: 'months' },
    { diffn: differenceInWeeks, stamp: 'weeks' },
    { diffn: differenceInDays, stamp: 'days' },
    { diffn: differenceInHours, stamp: 'hours' },
    { diffn: differenceInMinutes, stamp: 'minutes' },
    { diffn: differenceInSeconds, stamp: 'seconds' },
];

/**
 * Get DateTime interval \
 * For examples:
 * ```ts
 * // current datetime is: 2024-05-05 00:00:00
 * const res1 = getDateTimeInterval("2020-05-05 00:00:00")
 * assert.equal(res1, { value: 4, stamp: 'years' })
 *
 * const res1 = getDateTimeInterval("2024-02-05 00:00:00")
 * assert.equal(res1, { value: 3, stamp: 'months' })
 * ```
 * @param targetDate for diff datetime
 * @returns largest time interval */
export function getDateTimeInterval(targetDate: string): DateTimeInterval | null {
    const current = new Date();
    const target = parseISO(targetDate);

    for (const { diffn, stamp } of DIFFS) {
        const value = diffn(current, target);
        if (value > 0) {
            return { value, stamp };
        }
    }
    return null;
}

/**
 * Parse the string into general DateTime format \
 * For examples:
 * ```ts
 * const res = formatDateTime("2024-01-30T15:30:27.626220600Z");
 * assert.equal(res, "2024-01-30 15:30:27")
 * ```
 * @param str target string
 * @returns new formated datetime */
export function formatDateTime(str: string): string {
    return format(parseISO(str), 'yyyy-MM-dd HH:mm:ss');
}
