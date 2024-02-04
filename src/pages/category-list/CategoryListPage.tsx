import { useTranslation } from 'react-i18next';
import { Grid, Stack, Skeleton, Title, Button, ScrollArea } from '@mantine/core';

import { ModalName, useCreateCategoryModal, useModelConfirmAction } from '@store/modal';
import { CategoryQuery } from '@api/category';

import { CategoryCard } from './components';

export default function CategoryListPage() {
    const { t } = useTranslation('pages', { keyPrefix: 'CategoryList' });
    const { data: categories, isLoading: isCategoriesLoading, refetch: categoriesRefetch } = CategoryQuery.useGetAll();
    const [_, { open }] = useCreateCategoryModal();

    // when import category success, refetch data
    useModelConfirmAction(ModalName.importCategory, categoriesRefetch);
    useModelConfirmAction(ModalName.CreateCategory, categoriesRefetch);

    return (
        <Stack>
            <Grid>
                <Grid.Col span={6}>
                    <Title order={3}>{t('title')}</Title>
                </Grid.Col>
                <Grid.Col span={6} style={{ textAlign: 'end' }}>
                    <Button onClick={open}>{t('create_category')}</Button>
                </Grid.Col>
            </Grid>
            <ScrollArea.Autosize type="auto">
                <Skeleton visible={isCategoriesLoading}>
                    <Grid w="inherit" p={10}>
                        {categories.map((val) => (
                            // eslint-disable-next-line object-curly-newline
                            <Grid.Col key={val.id} span={{ base: 12, xs: 6, sm: 6, md: 4, lg: 3 }}>
                                <CategoryCard key={val.id} data={val} />
                            </Grid.Col>
                        ))}
                    </Grid>
                </Skeleton>
            </ScrollArea.Autosize>
        </Stack>
    );
}
