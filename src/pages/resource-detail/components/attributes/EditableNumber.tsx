/* eslint-disable react/jsx-props-no-spreading */
import { BoxProps, Popover, Slider, Text } from '@mantine/core';
import { useCallback, useRef, useState } from 'react';

export interface EditableNumberProps extends BoxProps {
    value: number;

    max: number;

    min: number;

    /** display name */
    name: string;

    /** when text change vent
     * @param newValue new value of text */
    onChange: (newValue: number) => void;

    onEdit?: () => void;

    onEditFinished?: (newValue: number, isEdited: boolean) => void;
}

export function EditableNumber(props: EditableNumberProps) {
    const {
        value, max, min, name, onChange, onEdit, onEditFinished, ...boxProps
    } = props;
    const [inEdited, setInEdited] = useState<boolean>(false);
    const edited = useRef<boolean>(false);

    const handleClick = () => {
        setInEdited(true);
        if (onEdit) {
            onEdit();
        }
    };

    const handleBlur = useCallback((newVal: number) => {
        setInEdited(false);
        if (onEditFinished && inEdited) {
            onEditFinished(newVal, edited.current);
        }
        edited.current = false;
    }, [onEditFinished, inEdited]);

    // display value
    return (
        <Popover
            width="20rem"
            position="bottom"
            withArrow
            arrowSize={10}
            shadow="md"
            opened={inEdited}
            onClose={() => handleBlur(value)}
        >
            <Popover.Target>
                <Text title="double click to edit" onDoubleClick={handleClick} {...boxProps}>
                    {value}
                </Text>
            </Popover.Target>
            <Popover.Dropdown h="75px" p={10} pt={5} pb={5}>
                <Text>{name}</Text>
                <Slider
                    color="teal"
                    value={value}
                    max={max}
                    min={min}
                    onChange={(e) => {
                        onChange(e);
                        edited.current = true;
                    }}
                    marks={[
                        { value: min, label: 'start' },
                        { value: max, label: 'end' },
                    ]}
                />
            </Popover.Dropdown>
        </Popover>
    );
}
