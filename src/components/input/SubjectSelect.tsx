import { forwardRef, useMemo, useCallback } from 'react';
import { Group, Select, SelectItem, SelectItemProps, SelectProps, Text } from '@mantine/core';
import { SubjectQuery } from '@api/subject';

export interface SubjectSelectItem extends SelectItem {
    id: string;

    description: string;
}

interface ItemProps extends SelectItemProps {
    id: string;

    description: string;
}

const Item = forwardRef<HTMLDivElement, ItemProps>(
    ({ description, value, id, ...others }: ItemProps, ref) => (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <div ref={ref} {...others} key={id}>
            <Group>
                <Text>{value}</Text>
                <Text size="xs" color="dimmed">{description}</Text>
            </Group>
        </div>
    ),
);

export interface SubjectSelectProps extends Omit<SelectProps, 'itemComponent'|'data'> {
    onItemSelect: (data: SubjectSelectItem) => void;
}

export function SubjectSelect(props: SubjectSelectProps) {
    const { onItemSelect, ...selectProps } = props;
    const { data: subjectData } = SubjectQuery.useGetAll();

    const handleChanged = useCallback((value: string) => {
        // TODO: should use hash to speedup?
        const target = subjectData.find((val) => val.name === value);
        if (target) {
            onItemSelect({
                value:       target.name,
                id:          target.id,
                description: target.description,
            });
        }
    }, [subjectData, onItemSelect]);

    // Memo the autocomplete item
    const subjectSelectItem = useMemo(
        () => subjectData.map<SubjectSelectItem>((obj) => ({
            label:       obj.name,
            value:       obj.name,
            id:          obj.id,
            description: obj.description,
        })),
        [subjectData],
    );

    return (
        <Select
            // eslint-disable-next-line react/jsx-props-no-spreading
            {...selectProps}
            searchable
            placeholder="Select one subject"
            nothingFound="No Subject"
            onChange={handleChanged}
            data={subjectSelectItem}
            itemComponent={Item}
        />
    );
}
