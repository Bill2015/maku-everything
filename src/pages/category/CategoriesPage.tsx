import { useCallback } from 'react';
import { useDisclosure } from '@mantine/hooks';
import { Grid, Stack, Skeleton, Container, Title, Button } from '@mantine/core';
import { CategoryCreateDto, CategoryMutation, CategoryQuery } from '@api/category';
import { CategoryCard, CreateCategoryModal } from './components';

export function CategoriesPage() {
    const { data: categories, isLoading: isCategoriesLoading, refetch: categoriesRefetch } = CategoryQuery.useGetAll();
    const createCategory = CategoryMutation.useCreate();

    const [opened, { open, close }] = useDisclosure(false);

    const categoryItems = categories.map((val) => <CategoryCard key={val.id} data={val} />);

    const handleCreateConfirm = useCallback(async (data: CategoryCreateDto) => {
        const result = await createCategory.mutateAsync(data);
        close();
        categoriesRefetch();
        console.log(data);
        console.log(result);
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
                <Container fluid>
                    <Skeleton visible={isCategoriesLoading}>
                        <Grid>
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
