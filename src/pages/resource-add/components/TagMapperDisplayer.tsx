import { useCallback, useMemo } from 'react';
import {
    Button, Divider, Group, ScrollArea, Space, Stack, Text, Tooltip,
} from '@mantine/core';
import { IoIosSave } from 'react-icons/io';

import { TagSelectOptionValue } from '@components/input';
import { CategoryMutation } from '@api/category';
import { TagMapperItem } from './TagMapperItem';
import { useAddResourceContext, useTextTagMapperContext } from '../stores';

export interface TagMapperDisplayerProps {
    tagValues: TagSelectOptionValue[];

    global?: boolean;

    targetText?: string;
}

export function TagMapperDisplayer(props: TagMapperDisplayerProps) {
    const { tagValues, global = false, targetText = '' } = props;
    const updateMapper = CategoryMutation.useUpdateRule();
    const { category } = useAddResourceContext();
    const { modified, textMapperList: globalMapperList, textMapInsert, textMapDelete, setHighlightText } = useTextTagMapperContext();

    const textMapperList = useMemo(() => {
        if (global) {
            return globalMapperList;
        }
        return globalMapperList.filter((val) => targetText.toLowerCase().includes(val.key.toLowerCase()));
    }, [global, targetText, globalMapperList]);

    const handleUpdateMapper = useCallback(() => {
        if (!category) {
            return;
        }

        const items = globalMapperList
            .filter((val) => val.value)
            .map(({ key, value }) => ({ text: key, tag_id: value!.id }));

        updateMapper.mutateAsync({ id: category!.id, rules: items });
    }, [category, globalMapperList, updateMapper]);

    return (
        <Stack mih={0} gap={0}>
            <Group align="center" pt="md">
                <Text fw="bolder" c="violet" flex="0 0 30%">Target Text</Text>
                <Text fw="bolder" c="violet">Appended Tag</Text>
                { modified && (
                    <Tooltip label="The text mapper has been modified, do you wanna save it?">
                        <Button color="cyan" variant="subtle" ml="auto" h="1.55rem" fw="bold" pl={10} pr={10} onClick={handleUpdateMapper}>
                            <IoIosSave />
                            <Space w="sm" />
                            Update Changes
                        </Button>
                    </Tooltip>
                ) }
            </Group>
            <Divider />
            <ScrollArea.Autosize pt="sm" type="auto" style={{ textAlign: 'start' }}>
                <Stack gap={10} pr={20}>
                    {
                        textMapperList.map(({ key, value }) => (
                            <TagMapperItem
                                key={key}
                                text={key}
                                tagValues={tagValues}
                                defaultTagValue={value ? {
                                    value:       value.name,
                                    subjectName: value.subject_name,
                                    ...value,
                                } : null}
                                onMouseEnter={() => setHighlightText(key)}
                                onMouseLeave={() => setHighlightText('')}
                                onEdit={() => textMapInsert(key, null)}
                                onDelete={() => textMapDelete(key)}
                                onOptionSubmit={(option) => {
                                    textMapInsert(key, option ? {
                                        id:           option.id,
                                        name:         option.name,
                                        subject_name: option.subjectName,
                                    } : null);
                                }}
                            />
                        ))
                    }
                    <Space />
                </Stack>
            </ScrollArea.Autosize>
        </Stack>
    );
}
