import { useCallback, useState } from 'react';
import { Grid } from '@mantine/core';
import { useActiveCategoryRedux } from '@store/global';
import { CategoryQuery } from '@api/category';
import { TauriDropZone } from '@components/input';

import { TextTagMapperProvider, useAddResouces } from './hooks';
import { AddPageFunctionSide, AddPagePreviewSide } from './components';

import '@mantine/carousel/styles.css';
import classes from './ResourceAddPage.module.scss';

export function ResourceAddPageContent() {
    const { activeCategory } = useActiveCategoryRedux();
    const [activePath, setActivePath] = useState<string>('');
    const { data: category } = CategoryQuery.useGetById(activeCategory.id);
    const { handleDropFiles, resourceValues, getResourceValuesRef, handleDelete } = useAddResouces(category);

    // unknown bug, the resoure values have closure problem, i don't know why
    const handleSlideChange = useCallback((index: number) => {
        const value = getResourceValuesRef()[index];
        if (value) {
            setActivePath(value.file_path || value.url_path || '');
        }
    }, [getResourceValuesRef]);

    return (
        <Grid classNames={{ inner: classes.innerGrid }} miw={0} mih={0}>
            <Grid.Col p={0} span={{ lg: 6, sm: 12 }} mah="100%" ta="center" display="flex" pos="relative" style={{ justifyContent: 'center' }}>
                <AddPagePreviewSide data={resourceValues} onSlideChange={handleSlideChange} onDelete={handleDelete} />
            </Grid.Col>
            <Grid.Col span={{ lg: 6, sm: 12 }} mah="100%">
                <AddPageFunctionSide rootPath={category?.root_path || ''} text={activePath} />
            </Grid.Col>
            <TauriDropZone onDropFiles={handleDropFiles} />
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
