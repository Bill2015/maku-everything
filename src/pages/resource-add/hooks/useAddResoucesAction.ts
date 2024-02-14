import { useCallback } from 'react';
import { showNotification } from '@components/notification';
import { getNameAndExtFromPath, stringNormalize } from '@utils/urlParser';
import { ResourceCreateDto, ResourceMutation } from '@api/resource';
import { useAddResourceContext, useTextTagMapperContext } from '../stores';
import { ResourceCreateItem } from '../stores/add-resource.store';

export function useAddResoucesAction() {
    const { activeResource, category, resources, addResource, deleteResource } = useAddResourceContext();
    const { getResourceSpecificTags } = useTextTagMapperContext();
    const createResource = ResourceMutation.useCreate();

    // ------------------------------------------------
    /** drop file to upload */
    const addFromFiles = useCallback(async (filePaths: string[]) => {
        if (!category) {
            return;
        }

        const newValues: ResourceCreateItem[] = [];
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
                tags:            [],
                ignoreText:      new Set(),
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

        let newValue: ResourceCreateItem | null = null;
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
                tags:            [],
                ignoreText:      new Set(),
                file_path:       text,
            };
        }
        else if (text.startsWith('http')) {
            newValue = {
                name:            stringNormalize(text),
                belong_category: category.id,
                description:     '',
                tags:            [],
                ignoreText:      new Set(),
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

    // ------------------------------------------------
    const buildResourceCreateDto = useCallback((resource: ResourceCreateItem) => {
        const tags = getResourceSpecificTags(resource)
            .filter((val) => val.ignored === false)
            .map((val) => val.id)
            .concat(resource.tags.map((val) => val.id));

        const createResourceObj: ResourceCreateDto = {
            name:            resource.name,
            description:     resource.description,
            belong_category: resource.belong_category,
            file_path:       resource.file_path,
            url_path:        resource.url_path,
            tags:            tags,
        };
        return createResourceObj;
    }, [getResourceSpecificTags]);

    const saveActiveResource = useCallback(async () => {
        if (!activeResource) {
            return;
        }
        const resource = buildResourceCreateDto(activeResource.data);
        await createResource.mutateAsync(resource);
        deleteResource(activeResource.index);
    }, [activeResource, createResource, deleteResource, buildResourceCreateDto]);

    const saveAllResource = useCallback(async () => {
        for (const data of resources) {
            const resource = buildResourceCreateDto(data);
            // eslint-disable-next-line no-await-in-loop
            await createResource.mutateAsync(resource);
            deleteResource(0);
        }
    }, [resources, buildResourceCreateDto, createResource, deleteResource]);

    return {
        addFromFiles,
        addFromClipboard,
        saveActiveResource,
        saveAllResource,
    };
}
