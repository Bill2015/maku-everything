import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Grid, Card, Group, Text, Button, rem, Image } from '@mantine/core';
import { ResourceResDto } from '@api/resource';

export interface ResourceCardProps {
    data: ResourceResDto;

    onDetailClick: (data: ResourceResDto) => void;
}

export function ResourceCard(props: ResourceCardProps) {
    const { data, onDetailClick } = props;

    return (
        <Grid.Col lg={2} md={4} sm={6}>
            <Card shadow="sm" padding="lg" radius="md" withBorder>
                <Card.Section>
                    <Image src={convertFileSrc(data.file.root + data.file.path)} alt={data.name} height={180} />
                </Card.Section>

                <Group position="apart" mt="md" mb="xs">
                    <Text>{data.name}</Text>
                </Group>

                <Group position="apart" mt="md" mb="xs">
                    <Text>{data.description}</Text>
                </Group>

                <Group position="apart" mt="md" mb="xs" spacing={0}>
                    <Text style={{ width: '100%' }} size={rem(5)}>
                        Created At:
                        {data.created_at}
                    </Text>
                    <Text style={{ width: '100%' }} size={rem(5)}>
                        Updated At:
                        {data.updated_at}
                    </Text>
                </Group>

                <Group position="right">
                    <Button onClick={() => onDetailClick(data)}>Detail</Button>
                </Group>
            </Card>
        </Grid.Col>
    );
}
