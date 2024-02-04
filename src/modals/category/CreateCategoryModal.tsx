import { useCallback, useState } from 'react';
import { Button, Group, Input, Stack } from '@mantine/core';
import { CategoryCreateDto, CategoryMutation } from '@api/category';
import { BaseModal } from '@components/modal';
import { useCreateCategoryModal } from '@store/modal';
import { PathInput } from '@components/input';

const dafaultData: CategoryCreateDto = {
    name:        '',
    description: '',
    root_path:   '',
};
export function CreateCategoryModal() {
    const [data, setData] = useState<CategoryCreateDto>(dafaultData);
    const createCategory = CategoryMutation.useCreate();

    const [opened, { close, confirmClose, cancelClose }] = useCreateCategoryModal();

    const handleChange = (field: keyof CategoryCreateDto, val: string) => {
        setData((prev) => ({
            ...prev,
            [field]: val,
        }));
    };

    // When Create Confirm
    const handleCreateConfirm = useCallback(async () => {
        await createCategory.mutateAsync(data);
        setData(dafaultData);
        confirmClose();
    }, [createCategory, data, confirmClose]);

    return (
        // eslint-disable-next-line react/jsx-props-no-spreading
        <BaseModal opened={opened} onClose={close} title="Create New Category" centered>
            <Stack>
                <Input.Wrapper label="Name:">
                    <Input
                        placeholder="category name"
                        value={data.name}
                        onChange={(e) => handleChange('name', e.currentTarget.value)}
                    />
                </Input.Wrapper>
                <Input.Wrapper label="Description:">
                    <Input
                        placeholder="category description"
                        value={data.description}
                        onChange={(e) => handleChange('description', e.target.value)}
                    />
                </Input.Wrapper>
                <Input.Wrapper label="Root Path:">
                    <PathInput
                        directory
                        placeholder="root path"
                        value={data.root_path}
                        onChange={(val) => handleChange('root_path', val)}
                    />
                </Input.Wrapper>
                <Group justify="space-between">
                    <Button color="pink" onClick={cancelClose}>
                        Cancel
                    </Button>
                    <Button color="lime" onClick={handleCreateConfirm}>
                        Confirm
                    </Button>
                </Group>
            </Stack>
        </BaseModal>
    );
}
