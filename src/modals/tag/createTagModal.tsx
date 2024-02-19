import { useCallback, useState } from 'react';
import { Button, Group, Input, SegmentedControl, Stack, Title } from '@mantine/core';
import { TagAttrPayload, TagCreateDto, TagMutation } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';
import { useCreateTagModal } from '@store/modal';
import { SubjectSelect } from '@components/input';
import { SubTitle } from '@components/display';
import { SubjectQuery } from '@api/subject';
import { ErrorResBody } from '@api/common';
import { showNotification } from '@components/notification';
import { BaseModal } from '@components/modal';
import { TagAttributePanel } from './components';

const DEFAULT_VALUE: TagCreateDto = {
    name:            '',
    description:     '',
    belong_category: '',
    belong_subject:  '',
    tag_type:        'normal',
    attr:            {},
};

export function CreateTagModal() {
    const { activeCategory } = useActiveCategoryRedux();
    const [opened, { close, confirmClose, cancelClose }] = useCreateTagModal();
    const { data: subjectData } = SubjectQuery.useGetByCategory(activeCategory && activeCategory.id);

    const [data, setData] = useState<TagCreateDto>(DEFAULT_VALUE);
    const [belongSubject, setBelongSubject] = useState<{ value: string, id: string } | null>(null);
    const createTag = TagMutation.useCreate();

    const handleUpdateData = <T extends keyof TagCreateDto>(fields: T, value: TagCreateDto[T]) => {
        setData((prev) => ({ ...prev, [fields]: value }));
    };

    const handleCreateConfirm = useCallback(async () => {
        if (!belongSubject) {
            return;
        }

        try {
            await createTag.mutateAsync({
                ...data,
                belong_category: activeCategory.id,
                belong_subject:  belongSubject.id,
            });
            setData(DEFAULT_VALUE);
            confirmClose();
        }
        catch (e) {
            const error = e as ErrorResBody;
            showNotification('Create Tag Failed', error.message, 'error');
        }
    }, [createTag, data, belongSubject, activeCategory, confirmClose]);

    if (activeCategory === null) {
        return null;
    }
    return (
        <BaseModal opened={opened} onClose={close} title="Create New Tag" size="xl">
            <Group wrap="nowrap" align="stretch">
                <Stack flex={1} gap={15}>
                    <Title order={5}>Basic tag data</Title>
                    <Stack gap={3}>
                        <SubTitle>Belong Subject:</SubTitle>
                        <SubjectSelect
                            value={belongSubject?.value}
                            onClickResult={() => setBelongSubject(null)}
                            subjects={subjectData}
                            onItemSelect={setBelongSubject}
                        />
                    </Stack>

                    <Stack gap={3}>
                        <SubTitle>Name:</SubTitle>
                        <Input
                            placeholder="resource name"
                            value={data.name}
                            onChange={(e) => handleUpdateData('name', e.target.value)}
                        />
                    </Stack>

                    <Stack gap={3}>
                        <SubTitle>Description:</SubTitle>
                        <Input
                            placeholder="resource description"
                            value={data.description}
                            onChange={(e) => handleUpdateData('description', e.target.value)}
                        />
                    </Stack>

                    <Stack gap={3}>
                        <SubTitle>Tag Type:</SubTitle>
                        <SegmentedControl
                            color="blue"
                            defaultValue="normal"
                            maw="fit-content"
                            miw="20rem"
                            value={data.tag_type}
                            onChange={(value) => handleUpdateData('tag_type', value as TagCreateDto['tag_type'])}
                            data={Object.keys(TagAttrPayload.DEFAULT_VALUE)}
                        />
                    </Stack>

                    <Group justify="space-between">
                        <Button color="pink" onClick={cancelClose}>Cancel</Button>
                        <Button color="lime" onClick={handleCreateConfirm}>Confirm</Button>
                    </Group>
                </Stack>

                <TagAttributePanel.Root hidden={data.tag_type === 'normal'}>
                    <TagAttributePanel.Content
                        displayType={data.tag_type}
                        onAttributeChange={(val) => handleUpdateData('attr', val)}
                    />
                </TagAttributePanel.Root>
            </Group>
        </BaseModal>
    );
}
