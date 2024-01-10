import { PropsWithChildren, useState, useMemo, useRef, useEffect } from 'react';
import { Flex, Group, Pill, Stack, Text } from '@mantine/core';
import { millify }  from 'millify';
import { ResourceTagDto } from '@api/resource';
import { TagQuery, TagResDto } from '@api/tag';

import { ResourceTagSelect } from './ResourceTagSelect';

import classes from './ResourceTagStack.module.scss';

export interface ResourceTagGroupProps {
    subjectName: string;

    subjectId: string;

    tags: ResourceTagDto[];

    autoFocus: boolean;

    onSelectNewTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;

    onRemoveExistTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;
}

export function ResourceTagGroup(props: ResourceTagGroupProps) {
    const { subjectName, autoFocus, subjectId, tags, onSelectNewTag, onRemoveExistTag } = props;
    const selectRef = useRef<HTMLInputElement>(null);
    const [searchValue, setSearchValue] = useState<string>('');
    const [selectValue, setSelectValue] = useState<string>('');
    const { data: subjectTags } = TagQuery.useGetSubjectTags(subjectId);

    // When created, auto focus the add tag input
    useEffect(() => {
        if (autoFocus && selectRef.current) {
            selectRef.current?.focus();
        }
    }, [autoFocus]);

    const handleTagSelect = (value: TagResDto | undefined) => {
        if (value) {
            onSelectNewTag({ id: value.id, name: value.name });
            // A small timeout for clear input value
            setTimeout(() => {
                setSelectValue('');
                setSearchValue('');
            }, 1);
        }
    };

    const itemChip = tags.map((val) => (
        <Pill
            classNames={{ root: classes.pillRoot, label: classes.pilllabel }}
            withRemoveButton
            key={val.id}
            onRemove={() => onRemoveExistTag({ id: val.id, name: val.name })}
        >
            {val.name}
            <Text size="xs" opacity="0.6" pl={5} component="span">
                {`(${millify(val.tagged_count)})`}
            </Text>
        </Pill>
    ));

    const selectableTags = useMemo(() => subjectTags
        .filter((tag) => !tags.find((obj) => obj.id === tag.id)), [tags, subjectTags]);

    return (
        <Flex direction="column">
            <Text fz="md" c="indigo" pb={3}>{subjectName}</Text>
            <Group gap="xs">
                {itemChip}
                <ResourceTagSelect
                    placeholder="+"
                    ref={selectRef}
                    rightSectionWidth={0}
                    data={selectableTags}
                    value={selectValue}
                    searchValue={searchValue}
                    onSearchChange={(e) => setSearchValue(e)}
                    onItemSelect={handleTagSelect}
                />
            </Group>
        </Flex>
    );
}

export interface ResourceTagStackProps extends PropsWithChildren { }

export function ResourceTagStack(props: ResourceTagStackProps) {
    const { children } = props;

    return (
        <Stack>
            {children}
        </Stack>
    );
}

ResourceTagStack.Group = ResourceTagGroup;
