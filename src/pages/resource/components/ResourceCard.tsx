import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Card, Group, Text, Button, rem, Tooltip, Divider } from '@mantine/core';
import { ResourceResDto } from '@api/resource';
import { LinkIcon, ResponsiveImage, YoutubeThumbnail } from '@components/display';

export interface ResourceCardProps {
    data: ResourceResDto;

    onDetailClick: (data: ResourceResDto) => void;
}

export function ResourceCard(props: ResourceCardProps) {
    const { data, onDetailClick } = props;

    return (
        <Card shadow="sm" padding="lg" radius="md" withBorder>

            <Card.Section>
                {
                    data.url && (
                        <LinkIcon
                            pos="absolute"
                            top="2%"
                            right="5%"
                            host={data.url!.host}
                            url={data.url!.full}
                        />
                    )
                }
                {
                    data.file === null
                        ? <YoutubeThumbnail url={data.url!.full} />
                        : <ResponsiveImage src={convertFileSrc(data.root_path + data.file!.path)} alt={data.name} width="100%" height="100%" />
                }
            </Card.Section>

            <Group justify="flex-start" mt="md" mb="xs">
                <Tooltip label={data.name} openDelay={500}>
                    <Text fw={500} truncate="end">{data.name}</Text>
                </Tooltip>
            </Group>

            <Divider />

            <Group justify="flex-start" mt="md" mb="xs">
                <Text>{data.description}</Text>
            </Group>

            <Group justify="flex-start" mt="md" mb="xs">
                <Text style={{ width: '100%' }} size={rem(5)}>
                    Created At:
                    {data.created_at}
                </Text>
                <Text style={{ width: '100%' }} size={rem(5)}>
                    Updated At:
                    {data.updated_at}
                </Text>
            </Group>

            <Group justify="flex-end">
                <Button onClick={() => onDetailClick(data)}>Detail</Button>
            </Group>
        </Card>
    );
}
