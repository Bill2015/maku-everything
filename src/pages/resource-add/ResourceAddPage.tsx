import { Grid, Skeleton } from '@mantine/core';
import { useHotkeys } from '@mantine/hooks';
import { useActiveCategoryRedux } from '@store/global';
import { CategoryQuery } from '@api/category';
import { TauriDropZone } from '@components/input';

import { AddResourceProvider } from './stores';
import { useAddResoucesAction, TextTagMapperProvider } from './hooks';
import { AddPageFunctionSide, AddPagePreviewSide } from './components';

import '@mantine/carousel/styles.css';
import classes from './ResourceAddPage.module.scss';

export function ResourceAddPageContent() {
    const { addFromFiles, addFromClipboard } = useAddResoucesAction();

    // on pasted the text
    useHotkeys([['ctrl+V', addFromClipboard]]);

    return (
        <Grid classNames={{ inner: classes.innerGrid }} miw={0} mih={0}>
            <Grid.Col p={0} span={{ lg: 6, sm: 12 }} mah="100%" ta="center" display="flex" pos="relative" style={{ justifyContent: 'center' }}>
                <AddPagePreviewSide />
            </Grid.Col>
            <Grid.Col span={{ lg: 6, sm: 12 }} mah="100%">
                <AddPageFunctionSide />
            </Grid.Col>
            <TauriDropZone onDropFiles={addFromFiles} />
        </Grid>
    );
}

export default function ResourceAddPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const { data: category } = CategoryQuery.useGetById(activeCategory.id);

    if (!category) {
        return (
            <Skeleton height="100%" mb="xl" />
        );
    }

    return (
        <AddResourceProvider category={category}>
            <TextTagMapperProvider>
                <ResourceAddPageContent />
            </TextTagMapperProvider>
        </AddResourceProvider>
    );
}
