import { useCallback, useState } from 'react';
import { Modal, Button, Grid, Input, Title } from '@mantine/core';
import { TagMutation } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';
import { useCreateTagModel } from '@store/modal';
import { SubjectSelect } from '@components/input';

export function CreateTagModal() {
    const { activeCategory } = useActiveCategoryRedux();
    const { opened, close } = useCreateTagModel();
    const [title, setTitle] = useState<string>('');
    const [belongSubject, setBelongSubject] = useState({ value: '', id: '' });
    const [description, setDescription] = useState<string>('');
    const createTag = TagMutation.useCreate();

    const handleCreateConfirm = useCallback(() => {
        if (activeCategory === null) {
            return;
        }
        createTag.mutateAsync({
            name:            title,
            description:     description,
            belong_category: activeCategory.id,
            belong_subject:  belongSubject.id,
        });
        setTitle('');
        setDescription('');
        close();
    }, [createTag, description, title, activeCategory, belongSubject, close]);

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
                    <Input disabled value={activeCategory.title} />
                </Grid.Col>
                <Grid.Col span={4}>
                    Belong Subject:
                </Grid.Col>
                <Grid.Col span={8}>
                    <SubjectSelect onItemSelect={setBelongSubject} />
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
