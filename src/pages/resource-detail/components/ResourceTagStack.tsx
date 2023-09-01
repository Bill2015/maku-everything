import { PropsWithChildren, useState, useMemo } from 'react';
import { ActionIcon, Badge, Flex, Group, Select, Stack, Text, rem } from '@mantine/core';
import { RxCross1 } from 'react-icons/rx';
import { ResourceMutation, ResourceTagDto } from '@api/resource';
import { TagQuery, TagResDto } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';

export interface ResourceTagGroupProps {
    resourceId: string;

    subjectName: string;

    subjectId: string;

    tags: ResourceTagDto[];

    onSelectNewTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;

    onRemoveExistTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;
}

export function ResourceTagGroup(props: ResourceTagGroupProps) {
    const { resourceId, subjectName, subjectId, tags,
        onSelectNewTag, onRemoveExistTag } = props;
    const { activeCategory } = useActiveCategoryRedux();
    const createResourceTag = ResourceMutation.useAddTag();
    const { data: subjectTags } = TagQuery.useGetSubjectTags(activeCategory!.id, subjectId);

    const handleSelectChange = (tagName: string) => {
        const tag = subjectTags.find((val) => val.name === tagName);
        if (tag) {
            onSelectNewTag({ id: tag.id, name: tag.name });
        }
    };

    const handleRemoveClick = (tagId: string, tagName: string) => {
        onRemoveExistTag({ id: tagId, name: tagName });
    };

    const itemChip = tags.map((val) => (
        <Badge
            pr={3}
            variant="outline"
            tt="initial"
            rightSection={(
                <ActionIcon
                    size="xs"
                    color="blue"
                    radius="xl"
                    variant="transparent"
                    onClick={() => handleRemoveClick(val.id, val.name)}
                >
                    <RxCross1 size={rem(10)} />
                </ActionIcon>
            )}
        >
            {val.name}
        </Badge>
    ));

    const selectableTags = useMemo(() => {
        return subjectTags
            .filter((tag) => !tags.find((obj) => obj.id === tag.id))
            .map((tag) => ({
                key:   tag.id,
                value: tag.name,
                label: tag.name,
            }))
    }, [tags, subjectTags]);

    return (
        <Flex direction="column">
            <Text fz="md" c="indigo">{subjectName}</Text>
            <Group>
                {itemChip}
                <Select
                    searchable
                    data={selectableTags}
                    onChange={handleSelectChange}
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
