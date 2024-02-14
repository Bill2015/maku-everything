import { useRef } from 'react';
import { TagComboSelect, TagComboSelectRef, TagSelectOptionValue } from '@components/input';
import { ActionIcon, Box, Flex, Group, Text } from '@mantine/core';
import { FaRegEdit } from 'react-icons/fa';
import { RxCross1 } from 'react-icons/rx';

import { TagTypography } from '@components/display';

import classes from './TagMapperItem.module.scss';

export interface TagMapperItemProps {
    text: string;

    tagOptions: TagSelectOptionValue[];

    defaultTagValue: TagSelectOptionValue | null;

    onMouseEnter: () => void;

    onMouseLeave: () => void;

    onOptionSubmit: (option: TagSelectOptionValue | null) => void;

    onEdit: () => void;

    onDelete: () => void;
}

export function TagMapperItem(props: TagMapperItemProps) {
    const {
        text, tagOptions, defaultTagValue,
        onMouseEnter, onMouseLeave, onOptionSubmit, onEdit, onDelete,
    } = props;
    const comboSelectRef = useRef<TagComboSelectRef>(null);

    return (
        <Flex
            gap={20}
            className={classes.root}
            onMouseEnter={onMouseEnter}
            onMouseLeave={onMouseLeave}
        >
            <Text flex="0 0 30%" style={{ wordBreak: 'break-all' }}>
                {text}
            </Text>
            <Group justify="space-between" flex="1">
                {defaultTagValue && <TagTypography name={defaultTagValue.name} subjectName={defaultTagValue.subjectName} />}
                {!defaultTagValue && (
                    <TagComboSelect
                        ref={comboSelectRef}
                        data={tagOptions}
                        defaultValue={defaultTagValue!}
                        onSubmitOptions={onOptionSubmit}
                    />
                )}
                <Box>
                    <ActionIcon
                        variant="transparent"
                        onClick={() => {
                            onEdit();
                            setTimeout(() => comboSelectRef.current!.getInputRef()!.focus(), 10);
                        }}
                    >
                        <FaRegEdit />
                    </ActionIcon>
                    <ActionIcon variant="transparent" c="red" onClick={onDelete}>
                        <RxCross1 />
                    </ActionIcon>
                </Box>
            </Group>
        </Flex>
    );
}
