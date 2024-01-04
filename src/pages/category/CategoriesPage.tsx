import { useCallback } from 'react';
import { useDisclosure } from '@mantine/hooks';
import { Grid, Stack, Skeleton, Container, Title, Button } from '@mantine/core';
import { useSnackbar } from 'notistack';

import { CategoryCreateDto, CategoryMutation, CategoryQuery, CategoryResDto } from '@api/category';
import { useActiveCategoryRedux } from '@store/global';
import { useCategoryNavigate } from '@router/navigateHook';

import { CategoryCard, CreateCategoryModal } from './components';

export default function CategoriesPage() {
    const { data: categories, isLoading: isCategoriesLoading, refetch: categoriesRefetch } = CategoryQuery.useGetAll();
    const { enqueueSnackbar } = useSnackbar();
    const { setActiveCategory } = useActiveCategoryRedux();
    const navigateCategoryTo = useCategoryNavigate();
    const createCategory = CategoryMutation.useCreate();

    const [opened, { open, close }] = useDisclosure(false);

    // When Load clicked
    const handleCateogryLoadClick = useCallback(async (data: CategoryResDto) => {
        enqueueSnackbar(`Success Load ${data.name}`, { variant: 'info' });
        setActiveCategory({ id: data.id, name: data.name });
        navigateCategoryTo(data.name);
    }, [enqueueSnackbar, setActiveCategory, navigateCategoryTo]);

    const categoryItems = categories.map((val) => <CategoryCard key={val.id} data={val} onLoadClick={handleCateogryLoadClick} />);

    // When Create Confirm
    const handleCreateConfirm = useCallback(async (data: CategoryCreateDto) => {
        const _ = await createCategory.mutateAsync(data);
        close();
        categoriesRefetch();
    }, [categoriesRefetch, close, createCategory]);

    return (
        <>
            <Stack spacing="lg">
                <Grid>
                    <Grid.Col span={6}>
                        <Title order={3}>Category</Title>
                    </Grid.Col>
                    <Grid.Col span={6} style={{ textAlign: 'end' }}>
                        <Button onClick={open}>Create Category</Button>
                    </Grid.Col>
                </Grid>
                <Container fluid style={{ textAlign: 'start', margin: 0 }}>
                    <Skeleton visible={isCategoriesLoading}>
                        <Grid align="flex-start">
                            {categoryItems}
                        </Grid>
                    </Skeleton>
                </Container>
            </Stack>
            <CreateCategoryModal
                opened={opened}
                onConfirm={handleCreateConfirm}
                onClose={close}
            />
        </>
    );
}
