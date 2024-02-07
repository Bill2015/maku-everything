import { useCallback } from 'react';
import { showNotification } from '@components/notification';
import { ResourceCreateDto } from '@api/resource';
import { getNameAndExtFromPath, stringNormalize } from '@utils/urlParser';
import { useAddResourceContext } from '../stores';

export function useAddResoucesAction() {
    const { category, resources, addResource } = useAddResourceContext();

    // ------------------------------------------------
    /** drop file to upload */
    const addFromFiles = useCallback(async (filePaths: string[]) => {
        if (!category) {
            return;
        }

        const newValues: ResourceCreateDto[] = [];
        const valueSet = new Set(resources.map((val) => val.file_path));
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
            addResource(newValues);
        }
    }, [category, resources, addResource]);

    // ------------------------------------------------
    /** Add Resource from clipboard */
    const addFromClipboard = useCallback(async () => {
        if (!category) {
            return;
        }
        const text = await navigator.clipboard.readText();
        if (!text) {
            return;
        }

        let newValue: ResourceCreateDto | null = null;
        const valueSet = new Set([
            ...resources.map((val) => val.file_path),
            ...resources.map((val) => val.url_path),
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
            addResource(newValue);
        }
    }, [category, resources, addResource]);

    return {
        addFromFiles,
        addFromClipboard,
    };
}
