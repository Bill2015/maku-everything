import { PropsWithChildren, useCallback, useMemo, useRef } from 'react';
import { Flex, Group, Space, Stack, Text, UnstyledButton } from '@mantine/core';
import { RxCross2 } from 'react-icons/rx';
import { TagComboSelect, TagComboSelectRef, TagSelectOptionValue } from '@components/input';
import { EditableText, TagTypography } from '@components/display';
import { ResourceCreateDto } from '@api/resource';
import { useAddResourceContext, useTextTagMapperContext } from '../stores';

import classes from './AttributePanel.module.scss';
import { TextTagValue } from '../stores/text-tag-mapper.store';

function SubTitle({ children }: PropsWithChildren) {
    return (
        <>
            <Space h="lg" />
            <Text c="dimmed" fw="bolder" fz="sm" opacity="0.6">{children}</Text>
        </>
    );
}

export interface AttributePanelProps {
    tagValues: TagSelectOptionValue[];
}

export function AttributePanel(props: AttributePanelProps) {
    const { tagValues } = props;
    const { activeResource, updateResource, updateResourceTag, updateResourceIgnoreText } = useAddResourceContext();
    const { textMapperList } = useTextTagMapperContext();
    const tagComboRef = useRef<TagComboSelectRef>(null);

    const handleUpdate = useCallback((fieldName: keyof ResourceCreateDto, newValue: string) => {
        updateResource(activeResource!.index, {
            ...activeResource!.data,
            [fieldName]: newValue,
        });
    }, [updateResource, activeResource]);

    const handleDeleteTag = useCallback((tagId: string) => {
        updateResourceTag(activeResource!.index, 'delete', tagId);
    }, [activeResource, updateResourceTag]);

    const hanldeIgnored = useCallback((text: string) => {
        if (activeResource!.data.ignoreText.has(text)) {
            updateResourceIgnoreText(activeResource!.index, 'delete', text);
            return;
        }
        updateResourceIgnoreText(activeResource!.index, 'add', text);
    }, [activeResource, updateResourceIgnoreText]);

    // prevent already added tag appear in tag combobox select
    const filteredTagValue = useMemo(() => {
        if (!activeResource) {
            return tagValues;
        }
        const tagSet = new Set(activeResource!.data.tags.map((val) => val.id));
        return tagValues.filter((val) => !tagSet.has(val.id));
    }, [activeResource, tagValues]);

    // auto generate tags
    const autoTagValue = useMemo(() => {
        const map: Map<string, (TextTagValue & { ignored: boolean })> = new Map();

        if (!activeResource) {
            return map;
        }
        const { data: resource } = activeResource;
        for (const { key, value } of textMapperList) {
            if (value && resource.name.toLowerCase().includes(key.toLowerCase())) {
                const ignored = resource.ignoreText.has(key);
                map.set(key, { ...value, ignored: ignored });
            }
        }
        return map;
    }, [activeResource, textMapperList]);

    if (!activeResource) {
        return <>Empty</>;
    }

    return (
        <Stack gap={0}>
            <SubTitle>Name</SubTitle>
            <EditableText
                key={activeResource.data.name}
                value={activeResource.data.name}
                name="name"
                onChange={(val) => handleUpdate('name', val)}
            />
            <SubTitle>Description</SubTitle>
            <EditableText
                value={activeResource.data.description}
                name="name"
                onChange={(val) => handleUpdate('description', val)}
            />
            <SubTitle>Auto Generate Tags</SubTitle>
            <Flex gap={10} wrap="wrap">
                {
                    Array.from(autoTagValue.entries()).map(([text, value]) => (
                        <Group component="span" key={value.id} gap={0} className={classes.tagpill} opacity={value.ignored ? '0.5' : '1.0'}>
                            <TagTypography
                                name={value.name}
                                subjectName={value.subject_name}
                                fontSize={0.8}
                                styles={{ main: { textDecoration: value.ignored ? 'line-through Crimson 2px' : 'none' } }}
                            />
                            <UnstyledButton onClick={() => hanldeIgnored(text)}>
                                <RxCross2 />
                            </UnstyledButton>
                        </Group>
                    ))
                }
            </Flex>
            <SubTitle>Pre-Added Tags</SubTitle>
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
            <Space h="sm" />
            <TagComboSelect
                ref={tagComboRef}
                data={filteredTagValue}
                dropDownMaxHeight="30vh"
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
            <Space h="20vh" />
        </Stack>
    );
}
