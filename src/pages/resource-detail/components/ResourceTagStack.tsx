import { PropsWithChildren, useState, useMemo } from 'react';
import {
    ActionIcon, Badge, Flex, Group, Stack, Text, createStyles, rem,
} from '@mantine/core';
import { RxCross1 } from 'react-icons/rx';
import { ResourceTagDto } from '@api/resource';
import { TagQuery, TagResDto } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';
import { ResourceTagSelect } from './ResourceTagSelect';

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

export interface ResourceTagGroupProps {
    subjectName: string;

    subjectId: string;

    tags: ResourceTagDto[];

    onSelectNewTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;

    onRemoveExistTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;
}

export function ResourceTagGroup(props: ResourceTagGroupProps) {
    const { subjectName, subjectId, tags, onSelectNewTag, onRemoveExistTag } = props;
    const { activeCategory } = useActiveCategoryRedux();
    const { classes: selectClasses } = useSelectStyle();
    const [selectInput, setSelectInput] = useState<string>('');
    const { data: subjectTags } = TagQuery.useGetSubjectTags(activeCategory!.id, subjectId);

    const handleTagSelect = (value: TagResDto | undefined) => {
        if (value) {
            onSelectNewTag({ id: value.id, name: value.name });
            setSelectInput('');
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

    const selectableTags = useMemo(() => subjectTags
        .filter((tag) => !tags.find((obj) => obj.id === tag.id)), [tags, subjectTags]);

    return (
        <Flex direction="column">
            <Text fz="md" c="indigo">{subjectName}</Text>
            <Group spacing="sm">
                {itemChip}
                <ResourceTagSelect
                    rightSectionWidth={0}
                    classNames={selectClasses}
                    data={selectableTags}
                    value={selectInput}
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
