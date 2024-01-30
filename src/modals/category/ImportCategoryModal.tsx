import { useCallback, useState } from 'react';
import { Modal, Title, Stack, Input, FileInput, Button, Group } from '@mantine/core';
import { useImportCategoryModal } from '@store/modal';
import { CategoryMutation } from '@api/category';
import { showNotification } from '@components/notification';
import { ErrorResBody } from '@api/common';

export function ImportCategoryModal() {
    const [opened, { close }] = useImportCategoryModal();

    const [rootPath, setRootPath] = useState<string>('');
    const [file, setFile] = useState<File | null>(null);
    const importCategory = CategoryMutation.useImport();

    const handleConfirm = useCallback(async () => {
        if (!file || !rootPath) {
            return;
        }
        try {
            const _ = await new Promise<string>((reslove, reject) => {
                const reader = new FileReader();
                reader.onloadend = async (ev) => {
                    importCategory.mutateAsync({ new_root_path: rootPath, data: ev.target!.result as string })
                        .then(reslove)
                        .catch(reject);
                };
                reader.readAsText(file);
            });
            showNotification('Import Category Successful', '', 'success');
        }
        catch (e) {
            const error = e as ErrorResBody;
            showNotification('Import Category Failed', error.message, 'error');
        }

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
