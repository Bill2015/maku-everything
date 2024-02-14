import { useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import {
    Button, Divider, Group, ScrollArea, Space, Stack, Text, Tooltip,
} from '@mantine/core';
import { IoIosSave } from 'react-icons/io';

import { TagSelectOptionValue } from '@components/input';
import { TagMapperItem } from './TagMapperItem';
import { useTextTagMapperContext } from '../stores';

export interface TagMapperDisplayerProps {
    tagOpitons: TagSelectOptionValue[];

    global?: boolean;

    targetText?: string;
}

export function TagMapperDisplayer(props: TagMapperDisplayerProps) {
    const { tagOpitons, global = false, targetText = '' } = props;
    const { t } = useTranslation('pages', { keyPrefix: 'resourceAdd.TagMapperDisplayer' });
    const {
        modified,
        textMapperList: globalMapperList,
        textMapInsert,
        textMapDelete,
        setHighlightText,
        handleUpdateMapper,
    } = useTextTagMapperContext();

    const textMapperList = useMemo(() => {
        if (global) {
            return globalMapperList;
        }
        return globalMapperList.filter((val) => targetText.toLowerCase().includes(val.key.toLowerCase()));
    }, [global, targetText, globalMapperList]);

    return (
        <Stack mih={0} gap={0}>
            <Group align="center" pt="md">
                <Text fw="bolder" c="violet" flex="0 0 30%">{t('target_text')}</Text>
                <Text fw="bolder" c="violet">{t('appended_tag')}</Text>
                { modified && (
                    <Tooltip label={t('modified_hint')}>
                        <Button color="cyan" variant="subtle" ml="auto" h="1.55rem" fw="bold" pl={10} pr={10} onClick={handleUpdateMapper}>
                            <IoIosSave />
                            <Space w="sm" />
                            {t('save_changes')}
                        </Button>
                    </Tooltip>
                ) }
            </Group>
            <Divider />
            <ScrollArea.Autosize pt="sm" type="auto" style={{ textAlign: 'start' }}>
                <Stack gap={10} pr={20}>
                    {
                        textMapperList.map(({ key, tagValue: value }) => (
                            <TagMapperItem
                                key={key}
                                text={key}
                                tagOptions={tagOpitons}
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
