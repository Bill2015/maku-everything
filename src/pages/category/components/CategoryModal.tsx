import { useCallback, useState } from 'react';
import { ModalProps, Button, Grid, Input } from '@mantine/core';
import { CategoryCreateDto } from '@api/category';
import { BaseModal } from '@components/modal';

export interface CreateCategoryModalProps extends ModalProps {
    onConfirm: (data: CategoryCreateDto) => void;
}

export function CreateCategoryModal(props: CreateCategoryModalProps) {
    const { onConfirm, ...modelProps } = props;
    const [name, setName] = useState<string>('');
    const [description, setDescription] = useState<string>('');
    const [rootPath, setRootPath] = useState<string>('');

    const handleCreateConfirm = useCallback(() => {
        setName('');
        setDescription('');
        onConfirm({
            name:        name,
            description: description,
            root_path:   rootPath,
        });
    }, [description, rootPath, name, onConfirm]);

    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <BaseModal {...modelProps} title="Create New Category" centered>
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
                <Grid.Col span={4}>
                    Root Path:
                </Grid.Col>
                <Grid.Col span={8}>
                    <Input placeholder="root path" value={rootPath} onChange={(e) => setRootPath(e.target.value)} />
                </Grid.Col>
                <Grid.Col span={6}>
                    <Button color="pink">Cancel</Button>
                </Grid.Col>
                <Grid.Col span={6} style={{ textAlign: 'end' }}>
                    <Button color="lime" onClick={handleCreateConfirm}>Confirm</Button>
                </Grid.Col>
            </Grid>
        </BaseModal>
    );
}
