import { useCallback } from 'react';
import { Box, Stack, Title, Skeleton, ScrollArea, Group } from '@mantine/core';

import { useActiveCategoryRedux } from '@store/global';
import { useResourceDetailNavigate } from '@router/navigateHook';
import { ResourceMutation, ResourceQuery, ResourceResDto } from '@api/resource';
import { TauriDropZone } from '@components/input';
import { StackGrid } from '@components/layout';

import { ResourceCard } from './components/ResourceCard';

export default function ResourcesPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const navigateResourceTo = useResourceDetailNavigate();
    const {
        data: resourceData,
        isFetching: isResourceFetching,
        refetch: resourceRefetch,
    } = ResourceQuery.useGetByCategory(activeCategory.id);

    const createResource = ResourceMutation.useCreate();

    // When Resource Detail Click
    const handleResoruceDetail = useCallback(async (data: ResourceResDto) => {
        if (activeCategory) {
            navigateResourceTo(activeCategory.name, data.id);
        }
    }, [activeCategory, navigateResourceTo]);

    const resourceItems = resourceData.map((val) => (
        <ResourceCard
            key={val.id}
            data={val}
            onDetailClick={handleResoruceDetail}
        />
    ));

    const onDropFiles = useCallback(async (filePaths: string[]) => {
        if (filePaths.length === 1) {
            const _ = await createResource.mutateAsync({
                name:            '',
                description:     '',
                belong_category: activeCategory.id,
                file_path:       filePaths[0],
                url_path:        '',
            });
            resourceRefetch();
        }
    }, [activeCategory, createResource, resourceRefetch]);

    if (activeCategory === null) {
        return <Box>A</Box>;
    }
    return (
        <>
            <TauriDropZone onDropFiles={onDropFiles} />
            <Stack gap="lg">
                <Group justify="space-between">
                    <Title order={3}>
                        Current Category:
                        {activeCategory.name}
                    </Title>
                </Group>
                <ScrollArea h="100%" style={{ textAlign: 'start', margin: 0 }}>
                    <Skeleton visible={isResourceFetching}>
                        <StackGrid w={270}>
                            {resourceItems}
                        </StackGrid>
                    </Skeleton>
                </ScrollArea>
            </Stack>
        </>
    );
}
