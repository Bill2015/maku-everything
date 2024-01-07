import React, { useMemo, forwardRef } from 'react';
import { ComboboxItem, Select, SelectProps } from '@mantine/core';
import { TagResDto } from '@api/tag';

import classes from './ResourceTagSelect.module.scss';

export interface ResourceTagSelectProps extends Omit<SelectProps, 'data'|'searchable'>{
    data: TagResDto[];

    onItemSelect?: (value: TagResDto | undefined) => void;
}

export const ResourceTagSelect = forwardRef<HTMLInputElement, ResourceTagSelectProps>((props: ResourceTagSelectProps, ref) => {
    const { data, onItemSelect, onChange, ...selectProps } = props;

    const selectableTags = useMemo(() => data
        .map((tag) => ({
            key:   tag.id,
            value: tag.name,
            label: tag.name,
        })), [data]);

    const hanldeChanged = (value: string | null, option: ComboboxItem) => {
        // default event
        if (onChange) {
            onChange(value, option);
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
            classNames={classes}
            rightSectionWidth={0}
            data={selectableTags}
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...selectProps}
            onChange={hanldeChanged}
            searchable
        />
    );
});
