import { useRef } from 'react';
import { TagComboSelect, TagComboSelectRef, TagSelectOptionValue } from '@components/input';
import { ActionIcon, Box, Flex, Group, Text } from '@mantine/core';
import { FaRegEdit } from 'react-icons/fa';
import { RxCross1 } from 'react-icons/rx';

import { TagTypography } from '@components/display';
import { useTextTagMapperContext } from '../stores';

import classes from './TagMapperItem.module.scss';

export interface TagMapperItemProps {
    text: string;

    tagValues: TagSelectOptionValue[];
}

export function TagMapperItem(props: TagMapperItemProps) {
    const { text, tagValues } = props;
    const { textMap, textMapInsert, textMapDelete, setHighlightText } = useTextTagMapperContext();
    const comboSelectRef = useRef<TagComboSelectRef>(null);

    const selectedTagData = tagValues.find((val) => val.id === textMap.get(text));

    return (
        <Flex
            gap={20}
            className={classes.root}
            onMouseEnter={() => setHighlightText(text)}
            onMouseLeave={() => setHighlightText('')}
        >
            <Text flex="0 0 30%" style={{ wordBreak: 'break-all' }}>
                {text}
            </Text>
            <Group justify="space-between" flex="1">
                {selectedTagData && <TagTypography name={selectedTagData.name} subjectName={selectedTagData.subjectName} />}
                {!selectedTagData && (
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
                )}
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
        </Flex>
    );
}
