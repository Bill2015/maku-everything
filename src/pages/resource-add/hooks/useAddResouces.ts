import { useCallback } from 'react';
import { useHotkeys } from '@mantine/hooks';
import { CategoryResDto } from '@api/category';
import { showNotification } from '@components/notification';
import { useStateRef } from '@hooks/life-hooks';

export type ResourcePreviewType = {
    local?: string;

    url?: string;
}

export function useAddResouces(category: CategoryResDto | null) {
    const [resourceValues, setResourceValues, getResourceValuesRef] = useStateRef<ResourcePreviewType[]>([]);

    // on pasted the text
    useHotkeys([['ctrl+V', useCallback(async () => {
        if (!category) {
            return;
        }
        const text = await navigator.clipboard.readText();
        if (!text) {
            return;
        }
        let newValue: ResourcePreviewType | null = null;
        const valueSet = new Set(resourceValues.map((val) => val.url));
        if (valueSet.has(text)) {
            showNotification('Invalid Resource', `${text} already added`, 'error');
        }
        else if (text.startsWith(category.root_path)) {
            newValue = { local: text };
        }
        else if (text.startsWith('http')) {
            newValue = { url: text };
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

        const newValues: ResourcePreviewType[] = [];
        const valueSet = new Set(resourceValues.map((val) => val.local));
        for (const filePath of filePaths) {
            if (!filePath.startsWith(category.root_path)) {
                showNotification('Invalid Resource', filePath, 'error');
                break;
            }
            if (valueSet.has(filePath)) {
                showNotification('Invalid Resource', `${filePath} already added`, 'error');
                break;
            }

            newValues.push({ local: filePath });
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
