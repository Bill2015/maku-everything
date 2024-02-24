import { PropsWithChildren, useCallback, useMemo, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Flex, Group, Space, Stack, Text, UnstyledButton } from '@mantine/core';
import { RxCross2 } from 'react-icons/rx';
import { TagComboSelect, TagComboSelectRef, TagSelectOptionValue } from '@components/input';
import { EditableText, TagTypography } from '@components/display';
import { ResourceCreateDto } from '@api/resource';
import { useAddResourceContext, useTextTagMapperContext } from '../stores';

import classes from './AttributePanel.module.scss';

function SubTitle({ children }: PropsWithChildren) {
    return (
        <>
            <Space h="lg" />
            <Text c="dimmed" fw="bolder" fz="sm" opacity="0.6">{children}</Text>
        </>
    );
}

export interface AttributePanelProps {
    tagOptions: TagSelectOptionValue[];
}

export function AttributePanel(props: AttributePanelProps) {
    const { tagOptions } = props;
    const { t } = useTranslation('pages', { keyPrefix: 'resourceAdd.AttributePanel' });
    const { activeResource, updateResource, updateResourceTag, updateResourceIgnoreText } = useAddResourceContext();
    const { getResourceSpecificTags } = useTextTagMapperContext();
    const tagComboRef = useRef<TagComboSelectRef>(null);
    const [name, setName] = useState<string>('');
    const [description, setDescription] = useState<string>('');

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
    const filteredTagOptions = useMemo(() => {
        if (!activeResource) {
            return tagOptions;
        }
        const tagSet = new Set(activeResource!.data.tags.map((val) => val.id));
        return tagOptions.filter((val) => !tagSet.has(val.id));
    }, [activeResource, tagOptions]);

    if (!activeResource) {
        return <>Empty</>;
    }

    return (
        <Stack gap={0}>
            <SubTitle>{t('name')}</SubTitle>
            <EditableText
                value={name || activeResource.data.name}
                name={t('name')}
                onEdit={() => setName(activeResource.data.name)}
                onChange={setName}
                onEditFinished={(val) => handleUpdate('name', val)}
            />
            <SubTitle>{t('description')}</SubTitle>
            <EditableText
                value={description || activeResource.data.description}
                name={t('description')}
                onEdit={() => setDescription(activeResource.data.description)}
                onChange={setDescription}
                onEditFinished={(val) => handleUpdate('description', val)}
            />
            <SubTitle>{t('auto_generate_tags')}</SubTitle>
            <Flex gap={10} wrap="wrap">
                {
                    getResourceSpecificTags(activeResource.data).map((value) => (
                        <Group component="span" key={value.id} gap={0} className={classes.tagpill} opacity={value.ignored ? '0.5' : '1.0'}>
                            <TagTypography
                                name={value.name}
                                subjectName={value.subject_name}
                                fontSize={0.8}
                                styles={{ main: { textDecoration: value.ignored ? 'line-through Crimson 2px' : 'none' } }}
                            />
                            <UnstyledButton onClick={() => hanldeIgnored(value.text)}>
                                <RxCross2 />
                            </UnstyledButton>
                        </Group>
                    ))
                }
            </Flex>
            <SubTitle>{t('pre-added_Tags')}</SubTitle>
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
                data={filteredTagOptions}
                dropDownMaxHeight="30vh"
                inputProps={{ placeholder: t('add_tag_here') }}
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
