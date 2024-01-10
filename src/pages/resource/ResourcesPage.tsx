import { useCallback } from 'react';
import { Box, Stack, Title, Skeleton, ScrollArea, Input, Divider } from '@mantine/core';
import { FaSearch } from 'react-icons/fa';

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
        <Stack gap="lg" p={0}>
            <Stack justify="space-between" pr={40} gap={0}>
                <Title order={2}>
                    {activeCategory.name}
                </Title>
                <Input style={{ flexGrow: 1 }} placeholder="search resource..." rightSection={<FaSearch />} />
                <Divider mt={10} />
            </Stack>
            <ScrollArea h="100%" style={{ textAlign: 'start', margin: 0 }}>
                <Skeleton visible={isResourceFetching}>
                    <StackGrid w={270}>
                        {resourceItems}
                    </StackGrid>
                </Skeleton>
                <TauriDropZone onDropFiles={onDropFiles} />
            </ScrollArea>
        </Stack>
    );
}
