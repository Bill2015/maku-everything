import { useCallback } from 'react';
import { useDisclosure } from '@mantine/hooks';
import { Grid, Stack, Skeleton, Title, Button, ScrollArea } from '@mantine/core';

import { ModalName, useModelConfirmAction } from '@store/modal';
import { CategoryCreateDto, CategoryMutation, CategoryQuery } from '@api/category';

import { CategoryCard, CreateCategoryModal } from './components';

export default function CategoriesPage() {
    const { data: categories, isLoading: isCategoriesLoading, refetch: categoriesRefetch } = CategoryQuery.useGetAll();
    const createCategory = CategoryMutation.useCreate();

    const [opened, { open, close }] = useDisclosure(false);

    // When Create Confirm
    const handleCreateConfirm = useCallback(async (data: CategoryCreateDto) => {
        await createCategory.mutateAsync(data);
        close();
        categoriesRefetch();
    }, [categoriesRefetch, close, createCategory]);

    // when import category success, refetch data
    useModelConfirmAction(ModalName.importCategory, categoriesRefetch);

    return (
        <>
            <Stack>
                <Grid>
                    <Grid.Col span={6}>
                        <Title order={3}>Category</Title>
                    </Grid.Col>
                    <Grid.Col span={6} style={{ textAlign: 'end' }}>
                        <Button onClick={open}>Create Category</Button>
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
            <CreateCategoryModal
                opened={opened}
                onConfirm={handleCreateConfirm}
                onClose={close}
            />
        </>
    );
}
