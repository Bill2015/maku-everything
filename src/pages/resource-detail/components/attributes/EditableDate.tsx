/* eslint-disable react/jsx-props-no-spreading */
import { useCallback, useRef, useState } from 'react';
import { BoxProps, Popover, Text } from '@mantine/core';

import { DatePicker } from '@mantine/dates';
import { formatDate, toDateTime } from '@utils/date';

export interface EditableDateProps extends BoxProps {
    value: string;

    /** display name */
    name: string;

    /** when text change vent
     * @param newValue new value of text */
    onChange: (newValue: string) => void;

    onEdit?: () => void;

    onEditFinished?: (newValue: string, isEdited: boolean) => void;
}

export function EditableDate(props: EditableDateProps) {
    const { value, name, onChange, onEdit, onEditFinished, ...boxProps } = props;
    const [inEdited, setInEdited] = useState<boolean>(false);
    const edited = useRef<boolean>(false);

    const handleClick = () => {
        setInEdited(true);
        if (onEdit) {
            onEdit();
        }
    };

    const handleBlur = useCallback((newVal: string) => {
        setInEdited(false);
        if (onEditFinished && inEdited) {
            onEditFinished(newVal, edited.current);
        }
        edited.current = false;
    }, [onEditFinished, inEdited]);

    // display value
    return (
        <Popover
            position="bottom"
            withArrow
            arrowSize={10}
            shadow="md"
            opened={inEdited}
            onClose={() => handleBlur(value)}
        >
            <Popover.Target>
                <Text title="double click to edit" onDoubleClick={handleClick} {...boxProps}>
                    {formatDate(value)}
                </Text>
            </Popover.Target>
            <Popover.Dropdown p={10} pt={5} pb={5}>
                <Text>{name}</Text>
                <DatePicker
                    value={toDateTime(value)}
                    onChange={(e) => {
                        if (e) {
                            onChange(e.toISOString());
                            edited.current = true;
                        }
                    }}
                />
            </Popover.Dropdown>
        </Popover>
    );
}
