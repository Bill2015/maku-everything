import { useCallback, useState } from 'react';
import { Stack, Title, Skeleton, ScrollArea, Divider } from '@mantine/core';

import { useActiveCategoryRedux } from '@store/global';
import { useResourceDetailNavigate } from '@router/navigateHook';
import { ResourceMutation, ResourceQuery, ResourceResDto } from '@api/resource';
import { ComplexSearchInput, TauriDropZone } from '@components/input';
import { StackGrid } from '@components/layout';
import { showNotification } from '@components/notification';
import { ModalName, useModelConfirmAction } from '@store/modal';
import { TagQuery } from '@api/tag';

import { ResourceCard } from './components/ResourceCard';

export default function ResourcesPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const navigateResourceTo = useResourceDetailNavigate();
    const [search, setSearch] = useState<string>('');
    const {
        data: resourceData,
        isFetching: isResourceFetching,
        refetch: resourceRefetch,
    } = ResourceQuery.useGetByCategory(activeCategory.id);

    const { data: searchResult } = ResourceQuery.useStringQuering(search, activeCategory.id, (error) => {
        showNotification('Search Failed', error.message, 'error');
    });

    const { data: tagData, refetch: tagRefetch } = TagQuery.useGetByCategory(activeCategory.id);

    const createResource = ResourceMutation.useCreate();

    // When Resource Detail Click
    const handleResoruceDetail = useCallback(async (data: ResourceResDto) => {
        if (activeCategory) {
            navigateResourceTo(activeCategory.name, data.id);
        }
    }, [activeCategory, navigateResourceTo]);

    // drop file to upload
    const onDropFiles = useCallback(async (filePaths: string[]) => {
        if (filePaths.length === 1) {
            const _ = await createResource.mutateAsync({
                name:            '',
                description:     '',
                belong_category: activeCategory.id,
                file_path:       filePaths[0],
            });
            resourceRefetch();
        }
    }, [activeCategory, createResource, resourceRefetch]);

    // when create the resource, refetch
    useModelConfirmAction(ModalName.CreateResource, () => {
        resourceRefetch();
    });

    // when create the tag, refetch
    useModelConfirmAction(ModalName.CreateTag, () => {
        tagRefetch();
    });

    if (activeCategory === null) {
        return <Title>Category Not Founded</Title>;
    }
    return (
        <Stack gap="lg" p={0}>
            <Stack justify="space-between" pr={40} gap={0}>
                <Title order={2}>
                    {activeCategory.name}
                </Title>
                <ComplexSearchInput
                    tags={tagData}
                    onSubmitSearch={(text) => setSearch(text)}
                    onClearSearch={() => setSearch('')}
                />
                <Divider mt={10} />
            </Stack>

            <ScrollArea h="100%" style={{ textAlign: 'start', margin: 0 }}>
                <Skeleton visible={isResourceFetching}>
                    <StackGrid w={270}>
                        {
                            (search ? searchResult : resourceData).map((val) => (
                                <ResourceCard
                                    key={val.id}
                                    data={val}
                                    onDetailClick={handleResoruceDetail}
                                />
                            ))
                        }
                    </StackGrid>
                </Skeleton>
                <TauriDropZone onDropFiles={onDropFiles} />
            </ScrollArea>
        </Stack>
    );
}
