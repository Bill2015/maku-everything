import { useCallback, useState } from 'react';
import { Modal, Button, Grid, Input, Title } from '@mantine/core';
import { SubjectMutation } from '@api/subject';
import { useActiveCategoryRedux } from '@store/global';
import { useCreateSubjectModel } from '@store/modal';

export function CreateSubjectModal() {
    const { activeCategory } = useActiveCategoryRedux();
    const { opened, close } = useCreateSubjectModel();
    const [title, setTitle] = useState<string>('');
    const [description, setDescription] = useState<string>('');
    const createSubject = SubjectMutation.useCreate();

    const handleCreateConfirm = useCallback(() => {
        if (activeCategory === null) {
            return;
        }
        createSubject.mutateAsync({
            name:            title,
            description:     description,
            belong_category: activeCategory.id,
        });
        setTitle('');
        setDescription('');
        close();
    }, [createSubject, description, title, activeCategory, close]);

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
                    <Input disabled value={activeCategory.title} />
                </Grid.Col>
                <Grid.Col span={4}>
                    Title:
                </Grid.Col>
                <Grid.Col span={8}>
                    <Input placeholder="resource title" value={title} onChange={(e) => setTitle(e.target.value)} />
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
