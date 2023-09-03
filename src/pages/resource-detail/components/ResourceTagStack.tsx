import { PropsWithChildren, useState, useMemo, useRef, useEffect } from 'react';
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

    autoFocus: boolean;

    onSelectNewTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;

    onRemoveExistTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;
}

export function ResourceTagGroup(props: ResourceTagGroupProps) {
    const { subjectName, autoFocus, subjectId, tags, onSelectNewTag, onRemoveExistTag } = props;
    const selectRef = useRef<HTMLInputElement>(null);
    const { activeCategory } = useActiveCategoryRedux();
    const { classes: selectClasses } = useSelectStyle();
    const [searchValue, setSearchValue] = useState<string>('');
    const [selectValue, setSelectValue] = useState<string>('');
    const { data: subjectTags } = TagQuery.useGetSubjectTags(activeCategory!.id, subjectId);

    // When created, auto focus the add tag input
    useEffect(() => {
        if (autoFocus && selectRef.current) {
            selectRef.current?.focus();
        }
    }, [autoFocus]);

    const handleTagSelect = (value: TagResDto | undefined) => {
        if (value) {
            onSelectNewTag({ id: value.id, name: value.name });
            setSelectValue('');
            setSearchValue('');
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
                    ref={selectRef}
                    rightSectionWidth={0}
                    classNames={selectClasses}
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
