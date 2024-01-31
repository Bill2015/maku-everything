import { useCallback, useState } from 'react';
import { Button, Grid, Input } from '@mantine/core';
import { SubjectMutation } from '@api/subject';
import { useActiveCategoryRedux } from '@store/global';
import { useCreateSubjectModal } from '@store/modal';
import { ErrorResBody } from '@api/common';
import { showNotification } from '@components/notification';
import { BaseModal } from '@components/modal';

export function CreateSubjectModal() {
    const { activeCategory } = useActiveCategoryRedux();
    const [opened, { close, confirmClose, cancelClose }] = useCreateSubjectModal();
    const [name, setName] = useState<string>('');
    const [description, setDescription] = useState<string>('');
    const createSubject = SubjectMutation.useCreate();

    const handleCreateConfirm = useCallback(async () => {
        if (activeCategory === null) {
            return;
        }
        try {
            await createSubject.mutateAsync({
                name:            name,
                description:     description,
                belong_category: activeCategory.id,
            });
            setName('');
            setDescription('');
            confirmClose();
        }
        catch (e) {
            const error = e as ErrorResBody;
            showNotification('Create Subject Failed', error.message, 'error');
        }
    }, [createSubject, description, name, activeCategory, confirmClose]);

    if (activeCategory === null) {
        return null;
    }
    return (
        <BaseModal opened={opened} onClose={close} title="Create New Subject" centered>
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
                    <Button color="pink" onClick={cancelClose}>Cancel</Button>
                </Grid.Col>
                <Grid.Col span={6} style={{ textAlign: 'end' }}>
                    <Button color="lime" onClick={handleCreateConfirm}>Confirm</Button>
                </Grid.Col>
            </Grid>
        </BaseModal>
    );
}
