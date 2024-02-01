import * as dayjs from 'dayjs';

export type DateTimeUnit = Extract<dayjs.OpUnitType, 'years' | 'months' | 'weeks' | 'days' | 'hours' | 'minutes' | 'seconds'>;

const DATE_STAMPS: DateTimeUnit[] = ['years', 'months', 'weeks', 'days', 'hours', 'minutes', 'seconds'];

export type DateTimeInterval = { value: number, stamp: DateTimeUnit };

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
    const current = dayjs();
    const target = dayjs(targetDate);

    for (const stamp of DATE_STAMPS) {
        const value = current.diff(target, stamp);
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
    return dayjs(str).format('YYYY-MM-DD HH:mm:ss');
}
