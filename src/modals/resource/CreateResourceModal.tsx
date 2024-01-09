import { useCallback, useState } from 'react';
import { Modal, Button, Grid, Input, Title } from '@mantine/core';
import { ResourceMutation } from '@api/resource';
import { useActiveCategoryRedux } from '@store/global';
import { useCreateResourceModel } from '@store/modal';

export function CreateResourceModal() {
    const { activeCategory } = useActiveCategoryRedux();
    const { opened, close } = useCreateResourceModel();

    const [name, setName] = useState<string>('');
    const [description, setDescription] = useState<string>('');
    const [filePath, setFilePath] = useState<string>('');
    const [urlPath, setUrlPath] = useState<string>('');
    const createResource = ResourceMutation.useCreate();

    const handleCreateConfirm = useCallback(() => {
        setName('');
        setDescription('');
        createResource.mutateAsync({
            name:            name,
            description:     description,
            belong_category: activeCategory.id,
            file_path:       filePath,
            url_path:        urlPath,
        });
        close();
    }, [description, name, filePath, urlPath, activeCategory, createResource, close]);

    return (
        <Modal opened={opened} onClose={close} title={<Title order={2}>Create New Resource</Title>} centered>
            <Grid>
                <Grid.Col span={4}>
                    In:
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
                <Grid.Col span={12}>
                    File Path
                </Grid.Col>
                <Grid.Col span={12}>
                    <Input placeholder="resource file path" value={filePath} onChange={(e) => setFilePath(e.target.value)} />
                </Grid.Col>
                <Grid.Col span={12}>
                    URL Path
                </Grid.Col>
                <Grid.Col span={12}>
                    <Input placeholder="URL path" value={urlPath} onChange={(e) => setUrlPath(e.target.value)} />
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
