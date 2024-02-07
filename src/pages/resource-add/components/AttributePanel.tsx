import { useCallback } from 'react';
import { EditableText } from '@components/display';
import { Space, Stack, Text } from '@mantine/core';
import { ResourceCreateDto } from '@api/resource';
import { useAddResourceContext } from '../stores';

export interface AttributePanelProps {

}

export function AttributePanel() {
    const { activeResource, updateResource } = useAddResourceContext();

    const handleUpdate = useCallback((fieldName: keyof ResourceCreateDto, newValue: string) => {
        updateResource(activeResource!.index, {
            ...activeResource!.data,
            [fieldName]: newValue,
        });
    }, [updateResource, activeResource]);

    if (!activeResource) {
        return <>Empty</>;
    }

    return (
        <Stack gap={0}>
            <Space h="lg" />
            <Text c="dimmed" fw="bolder">Name</Text>
            <EditableText
                key={activeResource.data.name}
                value={activeResource.data.name}
                name="name"
                onChange={(val) => handleUpdate('name', val)}
            />
            <Space h="lg" />
            <Text c="dimmed" fw="bolder">Description</Text>
            <EditableText
                value={activeResource.data.description}
                name="name"
                onChange={(val) => handleUpdate('description', val)}
            />
        </Stack>
    );
}
