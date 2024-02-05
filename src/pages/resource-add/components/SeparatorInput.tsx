import { ActionIcon, Group, TagsInput } from '@mantine/core';
import { GrPowerReset } from 'react-icons/gr';

export interface SeparatorInputProps {
    value: string[];

    onChange: (values: string[]) => void;

    onReset: () => void;
}

export function SeparatorInput(props: SeparatorInputProps) {
    const { value, onChange, onReset } = props;

    return (
        <Group>
            <TagsInput
                flex={1}
                label="Press Enter to submit a separator"
                placeholder="Enter separator"
                value={value}
                onChange={onChange}
                onKeyDown={(e) => {
                    if (e.key !== 'Enter' || e.currentTarget.value !== ' ') {
                        return;
                    }
                    if (value.findIndex((val) => val === ' ') < 0) {
                        onChange([...value, ' ']);
                    }
                }}
            />
            <ActionIcon variant="outline" onClick={onReset}>
                <GrPowerReset />
            </ActionIcon>
        </Group>
    );
}
