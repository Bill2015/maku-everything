import { useCallback, useState } from 'react';
import { useParams } from 'react-router-dom';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { FcOpenedFolder } from 'react-icons/fc';
import {
    Box, Grid, Title, Text, Button, Flex, ScrollArea, Affix, rem, Divider,
} from '@mantine/core';

import { useActiveCategoryRedux } from '@store/global';
import { ResourceMutation, ResourceQuery } from '@api/resource';
import { ResourceDetailParam } from '@router/params';
import { useCreateSubjectModel, useCreateTagModel } from '@store/modal';
import { SubjectQuery } from '@api/subject';
import { ResponsiveImage } from '@components/display';
import { ReturnButton } from '@components/input';
import { ResourceAddSubjectSelect, ResourceTagStack } from './components';

import classes from './ResourceDetailPage.module.scss';

export default function ResourcesDetailPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const { resourceId } = useParams<ResourceDetailParam>();

    // when new subject group was created, use for auto focus
    const [newSubjectId, setNewSubjectId] = useState<string>('');

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
            exporeFile.mutateAsync(resourceData.file.root + resourceData.file.path);
        }
    }, [exporeFile, resourceData]);

    if (!resourceData) {
        return <Box>404 Not Found</Box>;
    }
    return (
        <>
            <Grid classNames={classes}>
                <Grid.Col p={0} span={{ lg: 5 }} ta="center" h="100%">
                    <ResponsiveImage
                        alt={resourceData.name}
                        src={convertFileSrc(resourceData.file.root + resourceData.file.path)}
                    />
                </Grid.Col>
                <Divider orientation="vertical" size="sm" />
                <Grid.Col span={{ lg: 6 }} h="100%">
                    <Flex gap="xs">
                        <Text fz="sm" c="dimmed" lh={2}>
                            {resourceData.file.root}
                            <Text component="span" fw={500} fz="sm">{resourceData.file.path}</Text>
                        </Text>
                        <Button onClick={handleExporeClick} variant="subtle" p={0} fz="1.45em">
                            <FcOpenedFolder />
                        </Button>
                    </Flex>
                    <ScrollArea mx="auto" h="100%" type="hover" classNames={{ scrollbar: 'mgra' }}>
                        <Grid w="100%">
                            <Grid.Col span={{ lg: 12 }}>
                                <Title order={2}>{resourceData.name}</Title>
                            </Grid.Col>

                            <Grid.Col span={{ lg: 12 }}>
                                <ResourceTagStack>
                                    {resourceTagData.map(({ subjectId, subjectName, tags }) => (
                                        <ResourceTagStack.Group
                                            autoFocus={subjectId === newSubjectId}
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

                            <Grid.Col span={{ lg: 12 }}>
                                <Button onClick={() => openSubject()} variant="subtle" p={0} fz="1.45em">
                                    Open Subject
                                </Button>
                            </Grid.Col>

                            <Grid.Col span={{ lg: 12 }}>
                                <Button onClick={() => openTag()} variant="subtle" p={0} fz="1.45em">
                                    Open Tag
                                </Button>
                            </Grid.Col>

                            <Grid.Col span={{ lg: 12 }} style={{ paddingBottom: '60px' }}>
                                <ResourceAddSubjectSelect
                                    subjects={subjects}
                                    exclude={existedSubject}
                                    onSelectNewTag={async (tag) => {
                                        await addResourceTag.mutateAsync({ id: resourceData.id, tag_id: tag.id });
                                        await resourceRefetch();
                                        // make sure auto focus
                                        setNewSubjectId(tag.belong_subject);
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
