import { useCallback, useMemo, useRef } from 'react';
import { Flex, Group, Space, Stack, Text, UnstyledButton } from '@mantine/core';
import { RxCross2 } from 'react-icons/rx';
import { TagComboSelect, TagComboSelectRef, TagSelectOptionValue } from '@components/input';
import { EditableText, TagTypography } from '@components/display';
import { ResourceCreateDto } from '@api/resource';
import { useAddResourceContext } from '../stores';

import classes from './AttributePanel.module.scss';

export interface AttributePanelProps {
    tagValues: TagSelectOptionValue[];
}

export function AttributePanel(props: AttributePanelProps) {
    const { tagValues } = props;
    const { activeResource, updateResource, updateResourceTag } = useAddResourceContext();
    const tagComboRef = useRef<TagComboSelectRef>(null);

    const handleUpdate = useCallback((fieldName: keyof ResourceCreateDto, newValue: string) => {
        updateResource(activeResource!.index, {
            ...activeResource!.data,
            [fieldName]: newValue,
        });
    }, [updateResource, activeResource]);

    const handleDeleteTag = useCallback((tagId: string) => {
        if (!activeResource) {
            return;
        }
        updateResourceTag(activeResource.index, 'delete', tagId);
    }, [activeResource, updateResourceTag]);

    // prevent already added tag appear in tag combobox select
    const filteredTagValue = useMemo(() => {
        if (!activeResource) {
            return tagValues;
        }
        const tagSet = new Set(activeResource!.data.tags.map((val) => val.id));
        return tagValues.filter((val) => !tagSet.has(val.id));
    }, [activeResource, tagValues]);

    if (!activeResource) {
        return <>Empty</>;
    }

    return (
        <Stack gap={0}>
            <Space h="lg" />
            <Text c="dimmed" fw="bolder">Name</Text>
            <EditableText
                key={activeResource.data.name}
                value={activeResource.data.name}
                name="name"
                onChange={(val) => handleUpdate('name', val)}
            />
            <Space h="lg" />
            <Text c="dimmed" fw="bolder">Description</Text>
            <EditableText
                value={activeResource.data.description}
                name="name"
                onChange={(val) => handleUpdate('description', val)}
            />
            <Space h="lg" />
            <Text c="dimmed" fw="bolder">Pre-Added Tags</Text>
            <Flex gap={10} wrap="wrap">
                {
                    activeResource.data.tags.map((value) => (
                        <Group component="span" key={value.id} gap={0} className={classes.tagpill}>
                            <TagTypography name={value.name} subjectName={value.subject_name} fontSize={0.8} />
                            <UnstyledButton onClick={() => handleDeleteTag(value.id)}>
                                <RxCross2 />
                            </UnstyledButton>
                        </Group>
                    ))
                }
            </Flex>
            <Space h="lg" />
            <TagComboSelect
                ref={tagComboRef}
                data={filteredTagValue}
                onSubmitOptions={(value) => {
                    if (!activeResource || !value) {
                        return;
                    }
                    updateResourceTag(activeResource.index, 'add', {
                        id:           value.id,
                        name:         value.name,
                        subject_name: value.subjectName,
                    });
                    tagComboRef.current!.clearInput();
                }}
            />
            <Space h="45vh" />
        </Stack>
    );
}
