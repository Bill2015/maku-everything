import { useCallback, useState } from 'react';
import { useParams } from 'react-router-dom';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import {
    Box, Grid, Text, Flex, ScrollArea, Affix, rem, Divider, Group,
} from '@mantine/core';

import { useActiveCategoryRedux } from '@store/global';
import { ModalName, useModelConfirmAction } from '@store/modal';
import { ResourceMutation, ResourceQuery, ResourceUpdateDto } from '@api/resource';
import { ResourceDetailParam } from '@router/params';
import { SubjectQuery } from '@api/subject';
import { ActionFileIcon, EditableText } from '@components/display';
import { ReturnButton } from '@components/input';
import { showNotification } from '@components/notification';
import { ResourceAddSubjectSelect, ResourceTagStack } from './components';
import { ResourceDisplay } from './components/ResourceDisplay';

import classes from './ResourceDetailPage.module.scss';

export default function ResourcesDetailPage() {
    const { activeCategory } = useActiveCategoryRedux();
    const { resourceId } = useParams<ResourceDetailParam>();

    // when new subject group was created, use for auto focus
    const [newSubjectId, setNewSubjectId] = useState<string>('');
    const [name, setName] = useState<string>('');
    const [description, setDescription] = useState<string>('');

    const updateResource = ResourceMutation.useUpdate();
    const addResourceTag = ResourceMutation.useAddTag();
    const removeResourceTag = ResourceMutation.useRemoveTag();
    const updateResourceTag = ResourceMutation.useUpdateTag();

    const {
        data: resourceData,
        subjects: existedSubject,
        tagMapData: resourceTagData,
        refetch: resourceRefetch,
    } = ResourceQuery.useGetDetail(resourceId as string);

    const { data: subjects } = SubjectQuery.useGetByCategory(activeCategory?.id);

    const handleResourceUpdate = useCallback(async (fieldName: keyof ResourceUpdateDto, newVal: string) => {
        if (resourceId) {
            await updateResource.mutateAsync({ id: resourceId, [fieldName]: newVal })
                .then(() => {
                    showNotification('Update Resource Successful', '', 'success');
                })
                .catch((e) => {
                    showNotification('Update Resource Failed', e.message, 'error');
                })
                .finally(() => resourceRefetch());
        }
    }, [resourceId, updateResource, resourceRefetch]);

    // refetch when create the new tag & subject
    useModelConfirmAction(ModalName.CreateSubject, resourceRefetch);
    useModelConfirmAction(ModalName.CreateTag, resourceRefetch);

    if (!resourceData) {
        return <Box>404 Not Found</Box>;
    }
    return (
        <>
            <Grid classNames={classes}>
                <Grid.Col p={0} span={{ lg: 6, sm: 12 }} mah="100%" ta="center" display="flex" style={{ justifyContent: 'center' }}>
                    <ResourceDisplay
                        name={resourceData.name}
                        havePath={!!resourceData.file}
                        haveUrl={!!resourceData.url}
                        host={resourceData.url?.host}
                        url={resourceData.url?.full}
                        filePath={convertFileSrc(`${resourceData.root_path}${resourceData.file?.path}`)}
                    />
                </Grid.Col>
                <Grid.Col span={{ lg: 6, sm: 12 }} h="100%">
                    <Group gap="xs" pos="relative">
                        <Flex fz="sm" c="dimmed" align="center" pr="60px" lh={2} style={{ flexWrap: 'wrap', wordBreak: 'break-all' }}>
                            { resourceData.file ? resourceData.root_path : resourceData.url?.full}
                            {
                                resourceData.file && <Text component="span" fw={500} fz="sm">{resourceData.file.path}</Text>
                            }
                        </Flex>
                        {
                            resourceData.file && (
                                <ActionFileIcon filePath={resourceData.root_path + resourceData.file.path} pos="absolute" right="30px" variant="subtle" p={0} fz="1.75em" />
                            )
                        }
                    </Group>
                    <ScrollArea.Autosize mx="auto" mah="600px" type="hover" classNames={{ scrollbar: 'mgra' }}>
                        <EditableText
                            name="name"
                            fz="1.5rem"
                            fw="bold"
                            value={name || resourceData.name}
                            onEdit={() => setName(resourceData.name)}
                            onChange={setName}
                            onEditFinished={(newVal, isEdited) => {
                                if (isEdited) {
                                    handleResourceUpdate('name', newVal);
                                }
                            }}
                        />
                        <EditableText
                            name="description"
                            fz="1rem"
                            opacity="0.5"
                            fw="initial"
                            value={description || resourceData.description}
                            onEdit={() => setDescription(resourceData.description)}
                            onEditFinished={(newVal, isEdited) => {
                                if (isEdited) {
                                    handleResourceUpdate('description', newVal);
                                }
                            }}
                            onChange={setDescription}
                        />
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
                                    onRemoveTag={async (tag) => {
                                        await removeResourceTag.mutateAsync({ id: resourceData.id, tag_id: tag.id });
                                        resourceRefetch();
                                    }}
                                    onUpdateTag={async (tag, attrVal) => {
                                        await updateResourceTag.mutateAsync({
                                            id:      resourceData.id,
                                            tag_id:  tag.id,
                                            attrval: attrVal,
                                        });
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
                        <Divider pb="50%" />
                    </ScrollArea.Autosize>
                </Grid.Col>
            </Grid>
            <Affix position={{ bottom: rem(20), right: rem(20) }}>
                <ReturnButton />
            </Affix>
        </>
    );
}
