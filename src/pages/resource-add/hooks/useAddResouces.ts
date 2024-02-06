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
        const newValues = [...resourceValues];
        const valueSet = new Set(resourceValues.map((val) => val.url));
        if (valueSet.has(text)) {
            showNotification('Invalid Resource', `${text} already added`, 'error');
        }
        else if (text.startsWith(category.root_path)) {
            newValues.push({ local: text });
        }
        else if (text.startsWith('http')) {
            newValues.push({ url: text });
        }
        else {
            showNotification('Invalid Resource', text, 'error');
        }
        setResourceValues(newValues);
    }, [category, resourceValues, setResourceValues])]]);

    // drop file to upload
    const onDropFiles = useCallback(async (filePaths: string[]) => {
        if (!category) {
            return;
        }

        const newValues = [...resourceValues];
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
        setResourceValues(newValues);
    }, [category, resourceValues, setResourceValues]);

    return {
        onDropFiles, resourceValues, getResourceValuesRef,
    };
}
