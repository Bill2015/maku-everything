import { useCallback } from 'react';
import { useParams } from 'react-router-dom';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { FcOpenedFolder } from 'react-icons/fc';
import { Box, Grid, Image, Title, Text, Button, Flex, Badge } from '@mantine/core';

import { useActiveCategoryRedux } from '@store/global';
import { ResourceMutation, ResourceQuery } from '@api/resource';
import { TagQuery } from '@api/tag';
import { ResourceDetailParam } from '@router/params';
import { useCreateSubjectModel, useCreateTagModel } from '@store/modal';
import { ResourceTagStack } from './components';

export default function ResourcesDetailPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const { resourceId } = useParams<ResourceDetailParam>();

    const exporeFile = ResourceMutation.useExporeFile();
    const addResourceTag = ResourceMutation.useAddTag();
    const removeResourceTag = ResourceMutation.useRemoveTag();

    const { data: resourceData, tagMapData: resourceTagData, refetch: resourceRefetch } = ResourceQuery.useGetDetail(resourceId as string);
    const { data: taglist } = TagQuery.useGetSubjectTags(activeCategory?.id, '');
    const { open: openSubject } = useCreateSubjectModel();
    const { open: openTag } = useCreateTagModel();

    const handleExporeClick = useCallback(() => {
        if (resourceData) {
            exporeFile.mutateAsync(resourceData.file.path);
        }
    }, [exporeFile, resourceData]);

    if (!resourceData) {
        return <Box>404 Not Found</Box>;
    }
    return (
        <Grid style={{ height: '100%' }}>
            <Grid.Col lg={6} ta="center">
                <Image src={convertFileSrc(resourceData.file.path)} />
            </Grid.Col>
            <Grid.Col lg={6}>
                <Grid>
                    <Grid.Col lg={12}>
                        <Flex gap="xs">
                            <Text fz="sm" c="dimmed" lh={2}>
                                {resourceData.file.path}
                            </Text>
                            <Button onClick={handleExporeClick} variant="subtle" compact p={0} fz="1.45em">
                                <FcOpenedFolder />
                            </Button>
                        </Flex>
                        <Title order={2}>{resourceData.title}</Title>
                    </Grid.Col>

                    <Grid.Col lg={12}>
                        <ResourceTagStack>
                            {resourceTagData.map(({ subjectId, subjectName, tags }) => (
                                <ResourceTagStack.Group
                                    key={subjectId}
                                    subjectId={subjectId}
                                    subjectName={subjectName}
                                    tags={tags}
                                    onSelectNewTag={async (tag) => {
                                        await addResourceTag.mutateAsync({ id: resourceData.id, tag_id: tag.id });
                                        resourceRefetch();
                                    }}
                                    onRemoveExistTag={async (tag) => {
                                        await removeResourceTag.mutateAsync({ id: resourceData.id, tag_id: tag.id });
                                        resourceRefetch();
                                    }}
                                />
                            ))}
                        </ResourceTagStack>
                    </Grid.Col>

                    <Grid.Col lg={12}>
                        <Button onClick={() => openSubject()} variant="subtle" compact p={0} fz="1.45em">
                            Open Subject
                        </Button>
                    </Grid.Col>

                    <Grid.Col lg={12}>
                        <Button onClick={() => openTag()} variant="subtle" compact p={0} fz="1.45em">
                            Open Tag
                        </Button>
                    </Grid.Col>

                    <Grid.Col lg={12}>
                        {taglist.map((val) => <Badge key={val.id}>{val.name}</Badge>)}
                    </Grid.Col>
                </Grid>
            </Grid.Col>
        </Grid>
    );
}
