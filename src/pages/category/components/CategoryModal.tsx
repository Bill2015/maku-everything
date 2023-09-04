import { useCallback, useState } from 'react';
import { Modal, ModalProps, Button, Grid, Input, Title } from '@mantine/core';
import { CategoryCreateDto } from '@api/category';

export interface CreateCategoryModalProps extends ModalProps {
    onConfirm: (data: CategoryCreateDto) => void;
}

export function CreateCategoryModal(props: CreateCategoryModalProps) {
    const { onConfirm, ...modelProps } = props;
    const [name, setName] = useState<string>('');
    const [description, setDescription] = useState<string>('');

    const handleCreateConfirm = useCallback(() => {
        setName('');
        setDescription('');
        onConfirm({ name: name, description: description });
    }, [description, name, onConfirm]);

    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <Modal {...modelProps} title={<Title order={2}>Create New Category</Title>} centered>
            <Grid>
                <Grid.Col span={4}>
                    Name:
                </Grid.Col>
                <Grid.Col span={8}>
                    <Input placeholder="category name" value={name} onChange={(e) => setName(e.target.value)} />
                </Grid.Col>
                <Grid.Col span={4}>
                    Description:
                </Grid.Col>
                <Grid.Col span={8}>
                    <Input placeholder="category description" value={description} onChange={(e) => setDescription(e.target.value)} />
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
