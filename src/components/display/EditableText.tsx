import { Badge, Box, BoxProps, Text } from '@mantine/core';
import { useCallback, useState } from 'react';

import classes from './EditableText.module.scss';

export interface EditableTextProps extends BoxProps {
    value: string;

    /** display name */
    name: string;

    /** when text change vent
     * @param newValue new value of text */
    onChange: (newValue: string) => void;
}

/**
 * Editable Text \
 * Use double click to edit the text
 *
 * **Note:** \
 * I use **`contentEditable`** instead of **`input`** or **`textarea`** \
 * Because input can't warp the text when too long \
 * And textarea can input newline, which is unnecessary in some fields */
export function EditableText(props: EditableTextProps) {
    const { value, name, onChange, ...boxProps } = props;
    const [inEdited, setInEdited] = useState<boolean>(false);
    const [newValue, setNewValue] = useState<string>(value);

    const handleClick = () => {
        setInEdited(true);
        setNewValue(value);
    };

    const handleBlur = useCallback((newVal: string) => {
        setNewValue(newVal);
        setInEdited(false);
        if (newVal !== value) {
            onChange(newVal);
        }
    }, [onChange, value]);

    if (inEdited) {
        return (
            <>
                <Box
                    contentEditable
                    className={classes.input}
                    suppressContentEditableWarning
                    onMouseEnter={(e) => e.currentTarget.focus()}
                    onFocus={(e) => {
                        const textNode = e.currentTarget.firstChild!;
                        // prevent blur not being triggered
                        setTimeout(() => {
                            const range = document.createRange();
                            range.setStart(textNode, 0);
                            range.setEnd(textNode, textNode.textContent!.length);

                            const selection = window.getSelection()!;
                            selection.removeAllRanges();
                            selection.addRange(range);
                        }, 1);
                    }}
                    onBlur={(e) => {
                        const child = e.currentTarget.lastChild;
                        handleBlur(child ? child.textContent! : '');
                    }}
                    onKeyDown={(e) => {
                        if (e.key === 'Escape' || e.key === 'Enter') {
                            e.preventDefault();
                            const child = e.currentTarget.lastChild;
                            handleBlur(child ? child.textContent! : '');
                        }
                    }}
                    onPaste={(e) => {
                        e.preventDefault();
                        setNewValue(e.clipboardData.getData('text/plain'));
                    }}
                    // eslint-disable-next-line react/jsx-props-no-spreading
                    {...boxProps}
                >
                    {newValue}
                </Box>
                <Badge color="indigo" pos="absolute" right={0} variant="outline">modifying</Badge>
            </>
        );
    }

    // value is empty
    if ((!value && !newValue) || ((value && !newValue))) {
        return (
            // eslint-disable-next-line react/jsx-props-no-spreading
            <Text className={classes.text} title="double click to edit" onDoubleClick={handleClick} {...boxProps}>
                {`no ${name} here...`}
            </Text>
        );
    }

    // display value
    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <Text className={classes.text} title="double click to edit" onDoubleClick={handleClick} {...boxProps}>
            {(value === newValue) ? value : newValue}
        </Text>
    );
}
