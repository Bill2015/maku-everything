import { useCallback, useState } from 'react';
import { useParams } from 'react-router-dom';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { FcOpenedFolder } from 'react-icons/fc';
import {
    Box, Grid, Title, Text, Button, Flex, ScrollArea, Affix, rem, Divider, Stack,
} from '@mantine/core';

import { useActiveCategoryRedux } from '@store/global';
import { ResourceMutation, ResourceQuery } from '@api/resource';
import { ResourceDetailParam } from '@router/params';
import { SubjectQuery } from '@api/subject';
import { ReturnButton } from '@components/input';
import { ResourceAddSubjectSelect, ResourceTagStack } from './components';
import { ResourceDisplay } from './components/ResourceDisplay';

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

    const handleExporeClick = useCallback(() => {
        if (resourceData && resourceData.file) {
            exporeFile.mutateAsync(resourceData.root_path + resourceData.file.path);
        }
    }, [exporeFile, resourceData]);

    if (!resourceData) {
        return <Box>404 Not Found</Box>;
    }
    return (
        <>
            <Grid classNames={classes}>
                <Grid.Col p={0} span={{ lg: 5, sm: 11 }} mah="100%" ta="center" display="flex" style={{ justifyContent: 'center' }}>
                    <ResourceDisplay
                        name={resourceData.name}
                        havePath={!!resourceData.file}
                        haveUrl={!!resourceData.url}
                        host={resourceData.url?.host}
                        url={resourceData.url?.full}
                        filePath={convertFileSrc(resourceData.root_path + resourceData.file?.path)}
                    />
                </Grid.Col>
                <Divider orientation="vertical" size="sm" />
                <Grid.Col span={{ lg: 6, sm: 12 }} h="100%">
                    <Flex gap="xs">
                        <Text fz="sm" c="dimmed" lh={2}>
                            {resourceData.root_path}
                            {
                                resourceData.file && <Text component="span" fw={500} fz="sm">{resourceData.file.path}</Text>
                            }
                        </Text>
                        <Button onClick={handleExporeClick} variant="subtle" p={0} fz="1.45em">
                            <FcOpenedFolder />
                        </Button>
                    </Flex>
                    <ScrollArea mx="auto" h="100%" type="hover" classNames={{ scrollbar: 'mgra' }}>
                        <Stack>
                            <Title order={2}>{resourceData.name}</Title>
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
                        </Stack>
                    </ScrollArea>
                </Grid.Col>
            </Grid>
            <Affix position={{ bottom: rem(20), right: rem(20) }}>
                <ReturnButton />
            </Affix>
        </>
    );
}
