import { useRef } from 'react';
import { TagComboSelect, TagComboSelectRef, TagSelectOptionValue } from '@components/input';
import { ActionIcon, Box, Divider, Grid, Group, Text } from '@mantine/core';
import { FaRegEdit } from 'react-icons/fa';
import { RxCross1 } from 'react-icons/rx';

import { TagTypography } from '@components/display';
import { useTextTagMapperContext } from '../hooks';

export interface TextItemProps {
    text: string;

    tagValues: TagSelectOptionValue[];
}

export function TextItem(props: TextItemProps) {
    const { text, tagValues } = props;
    const { textMap, textMapInsert, textMapDelete, setHighlightText } = useTextTagMapperContext();
    const comboSelectRef = useRef<TagComboSelectRef>(null);

    const selectedTagData = tagValues.find((val) => val.id === textMap.get(text));

    return (
        <>
            <Grid.Col span={4} display="flex" style={{ alignItems: 'center' }}>
                <Text
                    style={{ wordBreak: 'break-all' }}
                    onMouseEnter={() => setHighlightText(text)}
                    onMouseLeave={() => setHighlightText('')}
                >
                    {text}
                </Text>
            </Grid.Col>
            <Grid.Col span={8}>
                {
                    selectedTagData
                        ? (
                            <Group justify="space-between">
                                <TagTypography name={selectedTagData.name} subjectName={selectedTagData.subjectName} />
                                <Box>
                                    <ActionIcon
                                        variant="transparent"
                                        onClick={() => {
                                            textMapInsert(text, null);
                                            setTimeout(() => comboSelectRef.current!.getInputRef()!.focus(), 10);
                                        }}
                                    >
                                        <FaRegEdit />
                                    </ActionIcon>
                                    <ActionIcon variant="transparent" c="red" onClick={() => textMapDelete(text)}>
                                        <RxCross1 />
                                    </ActionIcon>
                                </Box>
                            </Group>
                        )
                        : (
                            <Group>
                                <TagComboSelect
                                    ref={comboSelectRef}
                                    data={tagValues}
                                    defaultValue={selectedTagData}
                                    onSubmitOptions={(option) => {
                                        if (option) {
                                            textMapInsert(text, option.id);
                                        }
                                        else {
                                            textMapDelete(text);
                                        }
                                    }}
                                />
                                <ActionIcon variant="transparent" c="red" onClick={() => textMapDelete(text)}>
                                    <RxCross1 />
                                </ActionIcon>
                            </Group>
                        )
                }
            </Grid.Col>
            <Grid.Col span={12}>
                <Divider opacity={0.25} />
            </Grid.Col>
        </>
    );
}
