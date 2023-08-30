import { useCallback, useState } from 'react';
import { Modal, ModalProps, Button, Grid, Input, Title } from '@mantine/core';
import { ResourceCreateDto } from '@api/resource';
import { ActiveCategory } from '@store/global';

export interface CreateResourceModalProps extends ModalProps {
    activeCategory: ActiveCategory;

    onConfirm: (data: ResourceCreateDto) => void;
}

export function CreateResourceModal(props: CreateResourceModalProps) {
    const { activeCategory, onConfirm, ...modelProps } = props;
    const [title, setTitle] = useState<string>('');
    const [description, setDescription] = useState<string>('');
    const [filePath, setFilePath] = useState<string>('');

    const handleCreateConfirm = useCallback(() => {
        setTitle('');
        setDescription('');
        onConfirm({
            title:           title,
            description:     description,
            belong_category: activeCategory.id,
            file_path:       filePath,
        });
    }, [description, title, filePath, activeCategory, onConfirm]);

    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <Modal {...modelProps} title={<Title order={2}>Create New Resource</Title>} centered>
            <Grid>
                <Grid.Col span={4}>
                    In:
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
                <Grid.Col span={12}>
                    File Path
                </Grid.Col>
                <Grid.Col span={12}>
                    <Input placeholder="resource file path" value={filePath} onChange={(e) => setFilePath(e.target.value)} />
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
