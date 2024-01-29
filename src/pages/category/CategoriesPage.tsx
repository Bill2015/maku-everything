import { useCallback } from 'react';
import { useDisclosure } from '@mantine/hooks';
import { Grid, Stack, Skeleton, Title, Button, ScrollArea, Flex } from '@mantine/core';

import { CategoryCreateDto, CategoryMutation, CategoryQuery } from '@api/category';

import { CategoryCard, CreateCategoryModal } from './components';

export default function CategoriesPage() {
    const { data: categories, isLoading: isCategoriesLoading, refetch: categoriesRefetch } = CategoryQuery.useGetAll();
    const createCategory = CategoryMutation.useCreate();

    const [opened, { open, close }] = useDisclosure(false);

    const categoryItems = categories.map((val) => <CategoryCard key={val.id} data={val} />);

    // When Create Confirm
    const handleCreateConfirm = useCallback(async (data: CategoryCreateDto) => {
        const _ = await createCategory.mutateAsync(data);
        close();
        categoriesRefetch();
    }, [categoriesRefetch, close, createCategory]);

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
                <ScrollArea style={{ textAlign: 'start', margin: 0 }}>
                    <Skeleton visible={isCategoriesLoading}>
                        <Flex align="flex-start" gap="sm" wrap="wrap">
                            {categoryItems}
                        </Flex>
                    </Skeleton>
                </ScrollArea>
            </Stack>
            <CreateCategoryModal
                opened={opened}
                onConfirm={handleCreateConfirm}
                onClose={close}
            />
        </>
    );
}
