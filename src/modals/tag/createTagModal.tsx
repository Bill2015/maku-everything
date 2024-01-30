import { useCallback, useState } from 'react';
import { Modal, Button, Grid, Input, Title, Text } from '@mantine/core';
import { TagMutation } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';
import { useCreateTagModal } from '@store/modal';
import { SubjectSelect } from '@components/input';
import { SubjectQuery } from '@api/subject';
import { ErrorResBody } from '@api/common';
import { showNotification } from '@components/notification';

export function CreateTagModal() {
    const { activeCategory } = useActiveCategoryRedux();
    const [opened, { close, confirmClose, cancelClose }] = useCreateTagModal();
    const { data: subjectData } = SubjectQuery.useGetByCategory(activeCategory && activeCategory.id);
    const [name, setName] = useState<string>('');
    const [belongSubject, setBelongSubject] = useState<{ value: string, id: string } | null>(null);
    const [description, setDescription] = useState<string>('');
    const createTag = TagMutation.useCreate();

    const handleCreateConfirm = useCallback(() => {
        if (activeCategory === null || belongSubject === null) {
            return;
        }
        try {
            createTag.mutateAsync({
                name:            name,
                description:     description,
                belong_category: activeCategory.id,
                belong_subject:  belongSubject.id,
            });
            setName('');
            setDescription('');
            confirmClose();
        }
        catch (e) {
            const error = e as ErrorResBody;
            showNotification('Create Tag Failed', error.message, 'error');
        }
    }, [createTag, description, name, activeCategory, belongSubject, confirmClose]);

    if (activeCategory === null) {
        return null;
    }
    return (
        <Modal opened={opened} onClose={close} title={<Title order={2}>Create New Tag</Title>} centered>
            <Grid>
                <Grid.Col span={4}>
                    Belong Category:
                </Grid.Col>
                <Grid.Col span={8}>
                    <Text>{activeCategory.name}</Text>
                </Grid.Col>
                <Grid.Col span={4}>
                    Belong Subject:
                </Grid.Col>
                <Grid.Col span={8}>
                    <SubjectSelect
                        value={belongSubject?.value}
                        onClickResult={() => setBelongSubject(null)}
                        subjects={subjectData}
                        onItemSelect={setBelongSubject}
                    />
                </Grid.Col>
                <Grid.Col span={4}>
                    Name:
                </Grid.Col>
                <Grid.Col span={8}>
                    <Input placeholder="resource name" value={name} onChange={(e) => setName(e.target.value)} />
                </Grid.Col>
                <Grid.Col span={4}>
                    Description:
                </Grid.Col>
                <Grid.Col span={8}>
                    <Input placeholder="resource description" value={description} onChange={(e) => setDescription(e.target.value)} />
                </Grid.Col>
                <Grid.Col span={6}>
                    <Button color="pink" onClick={cancelClose}>Cancel</Button>
                </Grid.Col>
                <Grid.Col span={6} style={{ textAlign: 'end' }}>
                    <Button color="lime" onClick={handleCreateConfirm}>Confirm</Button>
                </Grid.Col>
            </Grid>
        </Modal>
    );
}
