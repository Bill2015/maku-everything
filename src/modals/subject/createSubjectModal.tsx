import { useCallback, useState } from 'react';
import { Modal, Button, Grid, Input, Title } from '@mantine/core';
import { SubjectMutation } from '@api/subject';
import { useActiveCategoryRedux } from '@store/global';
import { useCreateSubjectModal } from '@store/modal';

export function CreateSubjectModal() {
    const { activeCategory } = useActiveCategoryRedux();
    const [opened, { close }] = useCreateSubjectModal();
    const [name, setName] = useState<string>('');
    const [description, setDescription] = useState<string>('');
    const createSubject = SubjectMutation.useCreate();

    const handleCreateConfirm = useCallback(() => {
        if (activeCategory === null) {
            return;
        }
        createSubject.mutateAsync({
            name:            name,
            description:     description,
            belong_category: activeCategory.id,
        });
        setName('');
        setDescription('');
        close();
    }, [createSubject, description, name, activeCategory, close]);

    if (activeCategory === null) {
        return null;
    }
    return (
        <Modal opened={opened} onClose={close} title={<Title order={2}>Create New Subject</Title>} centered>
            <Grid>
                <Grid.Col span={4}>
                    Belong:
                </Grid.Col>
                <Grid.Col span={8}>
                    <Input disabled value={activeCategory.name} />
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
                    <Button color="pink">Cancel</Button>
                </Grid.Col>
                <Grid.Col span={6} style={{ textAlign: 'end' }}>
                    <Button color="lime" onClick={handleCreateConfirm}>Confirm</Button>
                </Grid.Col>
            </Grid>
        </Modal>
    );
}
