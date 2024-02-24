import { useCallback, useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Badge, Box, BoxProps, ElementProps, Text } from '@mantine/core';

import classes from './EditableText.module.scss';

export interface ContentEditableProps extends BoxProps, Omit<ElementProps<'div', keyof BoxProps>, 'onChange'> {
    value: string;

    onChange: (value: string) => void;
}

export function ContentEditable(props: ContentEditableProps) {
    const { value, onChange, ...boxProps } = props;
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (ref && ref.current) {
            ref.current.textContent = value;
        }
    }, [value]);

    return (
        <Box
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...boxProps}
            ref={ref}
            onInput={(e) => onChange(e.currentTarget.textContent ?? '')}
            contentEditable
            suppressContentEditableWarning
        />
    );
}

export interface EditableTextProps extends BoxProps {
    value: string;

    /** display name */
    name: string;

    /** when text change vent
     * @param newValue new value of text */
    onChange: (newValue: string) => void;

    onEdit?: () => void;

    onEditFinished?: (newValue: string, isEdited: boolean) => void;
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
    const { value, name, onChange, onEdit, onEditFinished, ...boxProps } = props;
    const { t } = useTranslation('common', { keyPrefix: 'Display.EditableText' });
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
        if (onEditFinished) {
            onEditFinished(newVal, edited.current);
        }
        edited.current = false;
    }, [onEditFinished]);

    if (inEdited) {
        return (
            <Box pos="relative">
                <ContentEditable
                    value={value}
                    className={classes.input}
                    onMouseEnter={(e) => e.currentTarget.focus()}
                    onChange={(e) => {
                        onChange(e);
                        edited.current = true;
                    }}
                    onFocus={(e) => {
                        const textNode = e.currentTarget.firstChild!;
                        // prevent blur not being triggered
                        setTimeout(() => {
                            if (!textNode || !textNode.textContent) {
                                return;
                            }
                            const range = document.createRange();
                            range.setStart(textNode, 0);
                            range.setEnd(textNode, textNode.textContent!.length);

                            const selection = window.getSelection()!;
                            selection.removeAllRanges();
                            selection.addRange(range);
                        }, 1);
                    }}
                    onBlur={(e) => {
                        const text = e.currentTarget.textContent;
                        handleBlur(text ?? '');
                    }}
                    onKeyDown={(e) => {
                        if (e.key === 'Escape' || e.key === 'Enter') {
                            e.preventDefault();
                            const child = e.currentTarget.lastChild;
                            handleBlur(child ? child.textContent! : '');
                        }
                    }}
                    // eslint-disable-next-line react/jsx-props-no-spreading
                    {...boxProps}
                />
                <Badge color="indigo" pos="absolute" right={0} variant="outline" style={{ zIndex: 99 }}>
                    {t('modifying')}
                </Badge>
            </Box>
        );
    }

    // value is empty
    if (!value) {
        return (
            // eslint-disable-next-line react/jsx-props-no-spreading
            <Text className={classes.text} title={t('double_click_to_edit')} onDoubleClick={handleClick} {...boxProps}>
                {t('empty', { name })}
            </Text>
        );
    }

    // display value
    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <Text className={classes.text} title="double click to edit" onDoubleClick={handleClick} {...boxProps}>
            {value}
        </Text>
    );
}
