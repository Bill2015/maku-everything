import { useCallback } from 'react';
import { useParams } from 'react-router-dom';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { FcOpenedFolder } from 'react-icons/fc';
import { Box, Grid, Title, Text, Button, Flex, ScrollArea, Affix, rem } from '@mantine/core';

import { useActiveCategoryRedux } from '@store/global';
import { ResourceMutation, ResourceQuery } from '@api/resource';
import { ResourceDetailParam } from '@router/params';
import { useCreateSubjectModel, useCreateTagModel } from '@store/modal';
import { SubjectQuery } from '@api/subject';
import { ResourceAddSubjectSelect, ResourceTagStack } from './components';
import { ReturnButton } from '@components/input';

export default function ResourcesDetailPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const { resourceId } = useParams<ResourceDetailParam>();

    const exporeFile = ResourceMutation.useExporeFile();
    const addResourceTag = ResourceMutation.useAddTag();
    const removeResourceTag = ResourceMutation.useRemoveTag();

    const {
        data: resourceData,
        subjects: existedSubject,
        tagMapData: resourceTagData,
        refetch: resourceRefetch,
    } = ResourceQuery.useGetDetail(resourceId as string);
    const { data: subjects } = SubjectQuery.useGetByCategory(activeCategory?.id);
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
        <>
            <Grid mah="100%" h="100%">
                <Grid.Col p={0} lg={6} h="100%" ta="center">
                    <img alt="Iamge" style={{ maxHeight: '100%', maxWidth: '100%' }} src={convertFileSrc(resourceData.file.path)} />
                </Grid.Col>
                <Grid.Col p={0} lg={6} h="100%">
                    <Flex gap="xs">
                        <Text fz="sm" c="dimmed" lh={2}>
                            {resourceData.file.path}
                        </Text>
                        <Button onClick={handleExporeClick} variant="subtle" compact p={0} fz="1.45em">
                            <FcOpenedFolder />
                        </Button>
                    </Flex>
                    <ScrollArea mx="auto" h="100%" type="hover" classNames={{ scrollbar: 'mgra' }}>
                        <Grid w="100%">
                            <Grid.Col lg={12}>
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

                            <Grid.Col lg={12} style={{ paddingBottom: '60px' }}>
                                <ResourceAddSubjectSelect
                                    subjects={subjects}
                                    exclude={existedSubject}
                                    onSelectNewTag={async (tag) => {
                                        await addResourceTag.mutateAsync({ id: resourceData.id, tag_id: tag.id });
                                        resourceRefetch();
                                    }}
                                />
                            </Grid.Col>
                        </Grid>
                    </ScrollArea>
                </Grid.Col>
            </Grid>
            <Affix position={{ bottom: rem(20), right: rem(20) }}>
                <ReturnButton />
            </Affix>
        </>
    );
}
