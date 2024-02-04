import { useCallback, useState } from 'react';
import { Stack, Input, FileInput, Button, Group } from '@mantine/core';
import { useImportCategoryModal } from '@store/modal';
import { CategoryMutation } from '@api/category';
import { showNotification } from '@components/notification';
import { ErrorResBody } from '@api/common';
import { BaseModal } from '@components/modal';
import { PathInput } from '@components/input';

export function ImportCategoryModal() {
    const [opened, { close, confirmClose, cancelClose }] = useImportCategoryModal();

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
            confirmClose();
        }
        catch (e) {
            const error = e as ErrorResBody;
            showNotification('Import Category Failed', error.message, 'error');
        }
    }, [file, rootPath, confirmClose, importCategory]);

    return (
        <BaseModal opened={opened} onClose={close} title="Import New Category">
            <Stack>
                <Input.Wrapper required label="Root path">
                    <PathInput
                        placeholder="enter path here..."
                        value={rootPath}
                        onChange={(val) => setRootPath(val)}
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
                    <Button color="pink" onClick={cancelClose}>Cancel</Button>
                    <Button color="lime" onClick={handleConfirm}>Confirm</Button>
                </Group>
            </Stack>
        </BaseModal>
    );
}
