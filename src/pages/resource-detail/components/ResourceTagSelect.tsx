import React, { useState, useMemo, forwardRef } from 'react';
import { IoIosAddCircleOutline } from 'react-icons/io';
import { Select, SelectProps, createStyles } from '@mantine/core';
import { TagResDto } from '@api/tag';

const useSelectStyle = createStyles((_theme) => ({
    root: {
        flexGrow: 1,
        minWidth: '50%',
    },
    input: {
        border:          'none',
        backgroundColor: 'transparent',
        boxShadow:       'none',
        paddingLeft:     '0px!important',
    },
    icon: {
        width:      '20px',
        lineHeight: '2px',
        cursor:     'pointer',
        opacity:    '0.75',
    },
    rightSection: { display: 'none' },
}));

export interface ResourceTagSelectProps extends Omit<SelectProps, 'data'|'searchable'>{
    data: TagResDto[];

    onItemSelect?: (value: TagResDto | undefined) => void;

    onFocus?: (e: React.FocusEvent<HTMLInputElement, Element>) => void;

    onBlur?: (e: React.FocusEvent<HTMLInputElement, Element>) => void;
}

export const ResourceTagSelect = forwardRef<HTMLInputElement, ResourceTagSelectProps>((props: ResourceTagSelectProps, ref) => {
    const { data, onItemSelect, onChange, onFocus, onBlur, ...selectProps } = props;

    const { classes: selectClasses } = useSelectStyle();
    const [isSelectFocus, setSelectFocus] = useState<boolean>(false);

    const selectableTags = useMemo(() => data
        .map((tag) => ({
            key:   tag.id,
            value: tag.name,
            label: tag.name,
        })), [data]);

    const hanldeChanged = (value: string) => {
        // default event
        if (onChange) {
            onChange(value);
        }
        // custom event
        if (!onItemSelect) {
            return;
        }
        onItemSelect(data.find((val) => val.name === value));
    };

    return (
        <Select
            ref={ref}
            rightSectionWidth={0}
            onFocus={(e) => {
                setSelectFocus(true);
                if (onFocus) {
                    onFocus(e);
                }
            }}
            onBlur={(e) => {
                setSelectFocus(false);
                if (onBlur) {
                    onBlur(e);
                }
            }}
            icon={!isSelectFocus && <IoIosAddCircleOutline />}
            classNames={selectClasses}
            data={selectableTags}
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...selectProps}
            onChange={hanldeChanged}
            searchable
        />
    );
});
