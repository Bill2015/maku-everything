import { useCallback, useState } from 'react';
import { Modal, Title, Stack, Input, FileInput, Button, Group } from '@mantine/core';
import { useImportCategoryModel } from '@store/modal';
import { CategoryMutation } from '@api/category';

export function ImportCategoryModal() {
    const { opened, close } = useImportCategoryModel();

    const [rootPath, setRootPath] = useState<string>('');
    const [file, setFile] = useState<File | null>(null);
    const importCategory = CategoryMutation.useImport();

    const handleConfirm = useCallback(() => {
        if (!file || !rootPath) {
            return;
        }
        const reader = new FileReader();
        reader.onloadend = async (ev) => {
            await importCategory.mutateAsync({ new_root_path: rootPath, data: ev.target!.result as string });
        };
        reader.readAsText(file);
        close();
    }, [file, rootPath, close, importCategory]);

    return (
        <Modal opened={opened} onClose={close} title={<Title order={2}>Import New Category</Title>} centered>
            <Stack>
                <Input.Wrapper required label="Root path">
                    <Input
                        placeholder="enter path here..."
                        value={rootPath}
                        onChange={(e) => setRootPath(e.currentTarget.value)}
                    />
                </Input.Wrapper>

                <FileInput
                    required
                    label="Category data"
                    placeholder="Upload here..."
                    value={file}
                    accept=".maku"
                    onChange={setFile}
                />

                <Group justify="space-between">
                    <Button color="pink">Cancel</Button>
                    <Button color="lime" onClick={handleConfirm}>Confirm</Button>
                </Group>
            </Stack>
        </Modal>
    );
}
