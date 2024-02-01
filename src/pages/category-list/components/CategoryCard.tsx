import { useCallback } from 'react';
import {
    Card, Group, Text, Badge, Button, rem, Spoiler, Box, Title, Divider, Stack,
} from '@mantine/core';

import { CategoryMutation, CategoryResDto } from '@api/category';

import { ImagePreviewFlex } from '@components/layout';
import { ResourceThumbnailDisplayer, DateTimeDisplayer } from '@components/display';
import { showNotification } from '@components/notification';
import { useActiveCategoryRedux } from '@store/global';
import { useCategoryNavigate } from '@router/navigateHook';
import { ResourceQuery } from '@api/resource';

import classes from './CategoryCard.module.scss';
import { CategoryCardMenu } from './CategoryCardMenu';

export interface CategoryCardProps {
    data: CategoryResDto;
}

export function CategoryCard(props: CategoryCardProps) {
    const { data: categoryData } = props;
    const { setActiveCategory } = useActiveCategoryRedux();
    const navigateCategoryTo = useCategoryNavigate();
    const exportCategory = CategoryMutation.useExport();

    const { data: resourceData } = ResourceQuery.useQuerying({
        belong_category: categoryData.id,
        order_by:        'updated_at',
        limit:           5,
    });

    // on load category
    const handleLoadClick = useCallback(async () => {
        showNotification('Loaded Category', categoryData.name);
        setActiveCategory({ id: categoryData.id, name: categoryData.name });
        navigateCategoryTo(categoryData.name);
    }, [categoryData, setActiveCategory, navigateCategoryTo]);

    // on export click
    const handleExportClick = useCallback(async () => {
        await exportCategory.mutateAsync({ id: categoryData.id });
    }, [exportCategory, categoryData]);

    return (
        <Card shadow="sm" padding="md" pt="xs" radius="md" withBorder classNames={{ root: classes.card }}>
            <Card.Section>
                <ImagePreviewFlex>
                    {
                        resourceData.map((data) => <ResourceThumbnailDisplayer key={data.id} data={data} useBackgoundImg />)
                    }
                </ImagePreviewFlex>
            </Card.Section>

            <Title order={3} display="flex" pos="relative">
                <Box component="span" pr="sm">{categoryData.name}</Box>
                <Badge color="cyan" variant="light" mt={rem(8)}>{categoryData.resource_num}</Badge>
                <CategoryCardMenu
                    name={categoryData.name}
                    onExportclick={handleExportClick}
                />
            </Title>
            <Divider orientation="horizontal" size={1} />

            <Spoiler maxHeight={120} showLabel="Show more" hideLabel="Hide">
                <Box maw={300}>
                    <Text>{categoryData.description}</Text>
                </Box>
            </Spoiler>

            <Stack mt="md" mb="xs" gap="xs">
                <DateTimeDisplayer label="Created At:" date={categoryData.created_at} />
                <DateTimeDisplayer label="Updated At:" date={categoryData.updated_at} />
            </Stack>

            <Group>
                <Button onClick={handleLoadClick}>Load</Button>
            </Group>
        </Card>
    );
}
