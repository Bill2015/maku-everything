import { useCallback } from 'react';
import { Box, Stack, Grid, Title, Button, Container, Skeleton } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';

import { useActiveCategoryRedux } from '@store/global';
import { ResourceCreateDto, ResourceMutation, ResourceQuery } from '@api/resource';
import { ResourceCard } from './components/ResourceCard';
import { CreateResourceModal } from './components/ResourceModal';

export function ResourcesPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const { data: resourceData, isFetching: isResourceFetching, refetch: resourceRefetch } = ResourceQuery.useGetAll();
    const [opened, { open, close }] = useDisclosure(false);

    const createResource = ResourceMutation.useCreate();

    const resourceItems = resourceData.map((val) => <ResourceCard key={val.id} data={val} onDetailClick={() => {}} />);

    const handleCreateConfirm = useCallback(async (data: ResourceCreateDto) => {
        const result = await createResource.mutateAsync(data);
        close();
        resourceRefetch();
    }, [resourceRefetch, close, createResource]);

    if (activeCategory === null) {
        return <Box>A</Box>;
    }
    return (
        <>
            <Stack spacing="lg">
                <Grid>
                    <Grid.Col span={12}>
                        <Title order={3}>
                            Current Category:
                            {activeCategory.title}
                        </Title>
                    </Grid.Col>
                    <Grid.Col span={6}>
                        <Title order={3}>Resources</Title>
                    </Grid.Col>
                    <Grid.Col span={6} style={{ textAlign: 'end' }}>
                        <Button onClick={open}>Create Resources</Button>
                    </Grid.Col>
                </Grid>
                <Container fluid style={{ textAlign: 'start', margin: 0 }}>
                    <Skeleton visible={isResourceFetching}>
                        <Grid align="flex-start">
                            {resourceItems}
                        </Grid>
                    </Skeleton>
                </Container>
            </Stack>
            <CreateResourceModal
                opened={opened}
                activeCategory={activeCategory}
                onConfirm={handleCreateConfirm}
                onClose={close}
            />
        </>
    );
}
