import { PropsWithChildren, useState, useMemo, useRef, useEffect } from 'react';
import { Flex, Group, Stack, Text } from '@mantine/core';
import { ResourceTagAttrValDto, ResourceTagDto } from '@api/resource';
import { TagQuery, TagResDto } from '@api/tag';

import { ResourceTagSelect } from './ResourceTagSelect';

import { ResourceTagPill } from './ResourceTagPill';

export interface ResourceTagGroupProps {
    subjectName: string;

    subjectId: string;

    tags: ResourceTagDto[];

    autoFocus: boolean;

    onSelectNewTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;

    onRemoveTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;

    onUpdateTag: (tag: Pick<ResourceTagDto, 'id'|'name'>, attrVal: ResourceTagAttrValDto) => void;
}

export function ResourceTagGroup(props: ResourceTagGroupProps) {
    const { subjectName, autoFocus, subjectId, tags, onSelectNewTag, onRemoveTag, onUpdateTag } = props;
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

    const itemChip = tags.map((val) => <ResourceTagPill key={val.id} tag={val} onRemoveTag={onRemoveTag} onUpdateTag={onUpdateTag} />);

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
