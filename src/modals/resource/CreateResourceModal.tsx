import { useCallback, useState } from 'react';
import { Button, Grid, Input } from '@mantine/core';
import { ResourceMutation } from '@api/resource';
import { useActiveCategoryRedux } from '@store/global';
import { useCreateResourceModal } from '@store/modal';
import { ErrorResBody } from '@api/common';
import { showNotification } from '@components/notification';
import { BaseModal } from '@components/modal';
import { PathInput } from '@components/input';

export function CreateResourceModal() {
    const { activeCategory } = useActiveCategoryRedux();
    const [opened, { close, confirmClose, cancelClose }] = useCreateResourceModal();

    const [name, setName] = useState<string>('');
    const [description, setDescription] = useState<string>('');
    const [filePath, setFilePath] = useState<string>('');
    const [urlPath, setUrlPath] = useState<string>('');
    const createResource = ResourceMutation.useCreate();

    const handleCreateConfirm = useCallback(async () => {
        setName('');
        setDescription('');
        try {
            await createResource.mutateAsync({
                name:            name,
                description:     description,
                belong_category: activeCategory.id,
                file_path:       filePath,
                url_path:        urlPath,
            });
            confirmClose();
        }
        catch (e) {
            const error = e as ErrorResBody;
            showNotification('Create Resource Failed', error.message, 'error');
        }
    }, [description, name, filePath, urlPath, activeCategory, createResource, confirmClose]);

    return (
        <BaseModal opened={opened} onClose={close} title="Create New Resource" centered>
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
                    <PathInput placeholder="resource file path" value={filePath} onChange={(e) => setFilePath(e)} />
                </Grid.Col>
                <Grid.Col span={12}>
                    URL Path
                </Grid.Col>
                <Grid.Col span={12}>
                    <Input placeholder="URL path" value={urlPath} onChange={(e) => setUrlPath(e.target.value)} />
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
