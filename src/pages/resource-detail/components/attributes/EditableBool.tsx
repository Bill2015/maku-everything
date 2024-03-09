/* eslint-disable react/jsx-props-no-spreading */
import { Box, BoxProps, Checkbox } from '@mantine/core';
import { MdCircle } from 'react-icons/md';

import classes from './EditableBool.module.scss';

export interface EditableBoolProps extends BoxProps {
    value: boolean;

    /** display name */
    name: string;

    /** when text change vent
     * @param newValue new value of text */
    onChange: (newValue: boolean) => void;
}

export function EditableBool(props: EditableBoolProps) {
    const { value, name, onChange, ...boxProps } = props;

    return (
        <Checkbox
            {...boxProps}
            color="teal"
            size="0.9rem"
            name={name}
            variant="outline"
            // eslint-disable-next-line react/no-unstable-nested-components
            icon={({ className }) => (
                // eslint-disable-next-line object-curly-newline
                <Box className={className} style={{ display: 'flex', alignItems: 'center', width: '13px' }}>
                    <MdCircle />
                </Box>
            )}
            checked={value}
            classNames={{ root: classes.root, input: classes.input }}
            onChange={(e) => onChange(e.currentTarget.checked)}
        />
    );
}
