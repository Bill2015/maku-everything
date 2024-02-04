import { render, screen } from '@__test__/utils';
import { DateTimeDisplayer } from './DateTimeDisplayer';

describe('DateTimeDisplayer', () => {
    beforeAll(() => {
        jest.useFakeTimers().setSystemTime(new Date('2024-02-28T18:50:50.000+08:00'));
    });

    it.each<{ date: string, expected: string }>([
        { date: '2023-02-28T18:50:50.000+08:00', expected: '1 years' },
        { date: '2024-01-28T18:50:50.000+08:00', expected: '1 months' },
        { date: '2024-02-21T18:50:50.000+08:00', expected: '1 weeks' },
        { date: '2024-02-27T18:50:50.000+08:00', expected: '1 days' },
        { date: '2024-02-28T17:50:50.000+08:00', expected: '1 hours' },
        { date: '2024-02-28T18:49:50.000+08:00', expected: '1 minutes' },
        { date: '2024-02-28T18:50:49.000+08:00', expected: '1 seconds' },
        { date: '2024-02-28T18:50:50.000+08:00', expected: 'just_now' },
    ])(('should render "$expected" when date is "$date"'), ({ date, expected }) => {
        // Arrange
        render(<DateTimeDisplayer date={date} label="" />);

        // Act

        // Assert
        const _ = screen.getByText(expected);
    });

    it('should render label when given', () => {
        // Arrange
        const label = 'test label';
        const date = '2024-02-28T18:50:50.000+08:00';
        render(<DateTimeDisplayer date={date} label={label} />);

        // Act

        // Assert
        const _ = screen.getByText(label);
    });
});
