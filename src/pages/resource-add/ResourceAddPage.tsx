import { Grid } from '@mantine/core';
import { useHotkeys } from '@mantine/hooks';
import { useActiveCategoryRedux } from '@store/global';
import { CategoryQuery } from '@api/category';
import { TauriDropZone } from '@components/input';

import { TextTagMapperProvider, useAddResourceContext } from './hooks';
import { AddPageFunctionSide, AddPagePreviewSide } from './components';

import '@mantine/carousel/styles.css';
import classes from './ResourceAddPage.module.scss';

export function ResourceAddPageContent() {
    const { activeCategory } = useActiveCategoryRedux();
    const { data: category } = CategoryQuery.useGetById(activeCategory.id);
    const { addFromFiles, addFromClipboard, resources, deleteResource, setActiveResource, activeResource } = useAddResourceContext(category);

    // on pasted the text
    useHotkeys([['ctrl+V', addFromClipboard]]);

    return (
        <Grid classNames={{ inner: classes.innerGrid }} miw={0} mih={0}>
            <Grid.Col p={0} span={{ lg: 6, sm: 12 }} mah="100%" ta="center" display="flex" pos="relative" style={{ justifyContent: 'center' }}>
                <AddPagePreviewSide
                    data={resources}
                    activeResource={activeResource}
                    onSlideChange={setActiveResource}
                    onDelete={deleteResource}
                />
            </Grid.Col>
            <Grid.Col span={{ lg: 6, sm: 12 }} mah="100%">
                <AddPageFunctionSide category={category} activeResource={activeResource} />
            </Grid.Col>
            <TauriDropZone onDropFiles={addFromFiles} />
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
