import { useCallback } from 'react';
import { useHotkeys } from '@mantine/hooks';
import { CategoryResDto } from '@api/category';
import { showNotification } from '@components/notification';
import { useStateRef } from '@hooks/life-hooks';
import { ResourceCreateDto } from '@api/resource';
import { getNameAndExtFromPath, stringNormalize } from '@utils/urlParser';

export function useAddResouces(category: CategoryResDto | null) {
    const [resourceValues, setResourceValues, getResourceValuesRef] = useStateRef<ResourceCreateDto[]>([]);

    // on pasted the text
    useHotkeys([['ctrl+V', useCallback(async () => {
        if (!category) {
            return;
        }
        const text = await navigator.clipboard.readText();
        if (!text) {
            return;
        }
        let newValue: ResourceCreateDto | null = null;
        const valueSet = new Set([
            ...resourceValues.map((val) => val.file_path),
            ...resourceValues.map((val) => val.url_path),
        ]);
        if (valueSet.has(text)) {
            showNotification('Invalid Resource', `${text} already added`, 'error');
        }
        else if (text.startsWith(category.root_path)) {
            const [fileName, _] = getNameAndExtFromPath(text);
            newValue = {
                name:            fileName,
                belong_category: category.id,
                description:     '',
                file_path:       text,
            };
        }
        else if (text.startsWith('http')) {
            newValue = {
                name:            stringNormalize(text),
                belong_category: category.id,
                description:     '',
                url_path:        text,
            };
        }
        else {
            showNotification('Invalid Resource', text, 'error');
        }
        if (newValue) {
            setResourceValues((prev) => [...prev, newValue!]);
        }
    }, [category, resourceValues, setResourceValues])]]);

    // drop file to upload
    const handleDropFiles = useCallback(async (filePaths: string[]) => {
        if (!category) {
            return;
        }

        const newValues: ResourceCreateDto[] = [];
        const valueSet = new Set(resourceValues.map((val) => val.file_path));
        for (const filePath of filePaths) {
            if (!filePath.startsWith(category.root_path)) {
                showNotification('Invalid Resource', filePath, 'error');
                break;
            }
            if (valueSet.has(filePath)) {
                showNotification('Invalid Resource', `${filePath} already added`, 'error');
                break;
            }

            const [fileName, _] = getNameAndExtFromPath(filePath);
            newValues.push({
                name:            fileName,
                belong_category: category.id,
                description:     '',
                file_path:       filePath,
            });
        }
        if (newValues.length > 0) {
            setResourceValues((prev) => [...prev, ...newValues]);
        }
    }, [category, resourceValues, setResourceValues]);

    const handleDelete = useCallback((index: number) => {
        const newValues = [...resourceValues];
        newValues.splice(index, 1);
        setResourceValues(newValues);
    }, [resourceValues, setResourceValues]);

    return {
        handleDropFiles,
        resourceValues,
        getResourceValuesRef,
        handleDelete,
    };
}
