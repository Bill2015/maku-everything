import { Affix, Button, Grid, Group, Skeleton } from '@mantine/core';
import { useHotkeys } from '@mantine/hooks';
import { useActiveCategoryRedux } from '@store/global';
import { CategoryQuery } from '@api/category';
import { ReturnButton, TauriDropZone } from '@components/input';

import { useTranslation } from 'react-i18next';
import { AddResourceProvider, TextTagMapperProvider, useAddResourceContext } from './stores';
import { useAddResoucesAction } from './hooks';
import { AddPageFunctionSide, AddPagePreviewSide } from './components';

import '@mantine/carousel/styles.css';
import classes from './ResourceAddPage.module.scss';

export function ResourceAddPageContent() {
    const { t } = useTranslation('pages', { keyPrefix: 'resourceAdd.Main' });
    const { addFromFiles, addFromClipboard, saveActiveResource, saveAllResource } = useAddResoucesAction();
    const { resources } = useAddResourceContext();

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

            <Affix position={{ bottom: 30, right: 20 }}>
                <Group gap={5}>
                    { resources.length >= 2 && (
                        <Button color="green" variant="subtle" onClick={saveAllResource}>
                            {t('save_all')}
                        </Button>
                    )}
                    { resources.length >= 1 && (
                        <Button color="lime" variant="subtle" onClick={saveActiveResource}>
                            {t('save')}
                        </Button>
                    )}
                    <ReturnButton />
                </Group>
            </Affix>
            <TauriDropZone onDropFiles={addFromFiles} />
        </Grid>
    );
}

export default function ResourceAddPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const { data: category } = CategoryQuery.useGetById(activeCategory.id);
    const { data: categoryRules } = CategoryQuery.useGetRules(activeCategory.id);

    if (!category || !categoryRules) {
        return (
            <Skeleton height="100%" mb="xl" />
        );
    }

    return (
        <AddResourceProvider category={category}>
            <TextTagMapperProvider category={category} defaultTextMap={categoryRules.rules}>
                <ResourceAddPageContent />
            </TextTagMapperProvider>
        </AddResourceProvider>
    );
}
