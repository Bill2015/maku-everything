import { useCallback, useState } from 'react';
import { Grid } from '@mantine/core';
import { useActiveCategoryRedux } from '@store/global';
import { CategoryQuery } from '@api/category';
import { showNotification } from '@components/notification';
import { TauriDropZone } from '@components/input';
import { useStateRef } from '@hooks/life-hooks';

import { TextTagMapperProvider } from './hooks';
import { AddPageFunctionSide, AddPagePreviewSide, ResourcePreviewType } from './components';

import '@mantine/carousel/styles.css';
import classes from './ResourceAddPage.module.scss';

export function ResourceAddPageContent() {
    const { activeCategory } = useActiveCategoryRedux();
    const [activePath, setActivePath] = useState<string>('');
    const [resourceValues, setResourceValues, getResourceValuesRef] = useStateRef<ResourcePreviewType[]>([]);
    const { data: category } = CategoryQuery.useGetById(activeCategory.id);

    // drop file to upload
    const onDropFiles = useCallback(async (filePaths: string[]) => {
        if (!category) {
            return;
        }

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

            resourceValues.push({ local: filePath, index: 0 });
        }
        setResourceValues([...resourceValues]);
    }, [category, resourceValues, setResourceValues]);

    // unknown bug, the resoure values have closure problem, i don't know why
    const handleSlideChange = useCallback((index: number) => {
        const value = getResourceValuesRef()[index];
        if (value) {
            setActivePath(value.local || value.url || '');
        }
    }, [getResourceValuesRef]);

    return (
        <Grid classNames={{ inner: classes.innerGrid }} miw={0} mih={0}>
            <Grid.Col p={0} span={{ lg: 6, sm: 12 }} mah="100%" ta="center" display="flex">
                <AddPagePreviewSide data={resourceValues} onSlideChange={handleSlideChange} />
            </Grid.Col>
            <Grid.Col span={{ lg: 6, sm: 12 }} mah="100%">
                <AddPageFunctionSide text={activePath} />
            </Grid.Col>
            <TauriDropZone onDropFiles={onDropFiles} />
        </Grid>
    );
}

export default function ResourceAddPage() {
    return (
        <TextTagMapperProvider>
            <ResourceAddPageContent />
        </TextTagMapperProvider>
    );
}
