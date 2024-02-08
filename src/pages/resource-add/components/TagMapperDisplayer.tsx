import { useMemo } from 'react';
import { Divider, Group, ScrollArea, Space, Stack, Text } from '@mantine/core';
import { TagSelectOptionValue } from '@components/input';
import { TagMapperItem } from './TagMapperItem';
import { useTextTagMapperContext } from '../stores';

export interface TagMapperDisplayerProps {
    tagValues: TagSelectOptionValue[];

    global?: boolean;

    targetText?: string;
}

export function TagMapperDisplayer(props: TagMapperDisplayerProps) {
    const { tagValues, global = false, targetText = '' } = props;
    const { textMapperList: oldList, textMapInsert, textMapDelete, setHighlightText } = useTextTagMapperContext();

    const textMapperList = useMemo(() => {
        if (global) {
            return oldList;
        }
        return oldList.filter((val) => targetText.toLowerCase().includes(val.key.toLowerCase()));
    }, [global, targetText, oldList]);

    return (
        <Stack mih={0} gap={0}>
            <Group align="center" pt="sm">
                <Text fw="bolder" c="violet" flex="0 0 30%">Target Text</Text>
                <Text fw="bolder" c="violet">Appended Tag</Text>
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
                                defaultTagValue={value ? { value: value.name, ...value } : null}
                                onMouseEnter={() => setHighlightText(key)}
                                onMouseLeave={() => setHighlightText('')}
                                onEdit={() => textMapInsert(key, null)}
                                onDelete={() => textMapDelete(key)}
                                onOptionSubmit={(option) => {
                                    textMapInsert(key, option);
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
