import { Grid, Card, Group, Text, Badge, Button, rem } from '@mantine/core';
import { CategoryResDto } from '@api/category';

export interface CategoryCardProps {
    data: CategoryResDto;
}

export function CategoryCard(props: CategoryCardProps) {
    const { data } = props;

    return (
        <Grid.Col span={4}>
            <Card shadow="sm" padding="lg" radius="md" withBorder>
                <Group position="apart" mt="md" mb="xs">
                    <Text>{data.title}</Text>
                    <Badge color="cyan" variant="light">{data.resource_num}</Badge>
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
                    <Button>Load</Button>
                </Group>
            </Card>
        </Grid.Col>
    );
}
