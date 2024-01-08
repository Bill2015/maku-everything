import {
    Card, Group, Text, Badge, Button, rem, Spoiler, Box, Title, Divider,
} from '@mantine/core';
import { CategoryResDto } from '@api/category';

import classes from './CategoryCard.module.scss';

export interface CategoryCardProps {
    data: CategoryResDto;
    onLoadClick: (data: CategoryResDto) => void;
}

export function CategoryCard(props: CategoryCardProps) {
    const { data, onLoadClick } = props;

    return (
        <Card shadow="sm" padding="md" pt="xs" radius="md" withBorder classNames={{ root: classes.card }}>
            <Title order={3} display="flex">
                <Box component="span" pr="sm">{data.name}</Box>
                <Badge color="cyan" variant="light" mt={rem(8)}>{data.resource_num}</Badge>
            </Title>
            <Divider orientation="horizontal" size={1} />

            <Spoiler maxHeight={120} showLabel="Show more" hideLabel="Hide">
                <Box maw={300}>
                    <Text>{data.description}</Text>
                </Box>
            </Spoiler>

            <Group mt="md" mb="xs">
                <Text style={{ width: '100%' }} size={rem(5)}>
                    Created At:
                    {data.created_at}
                </Text>
                <Text style={{ width: '100%' }} size={rem(5)}>
                    Updated At:
                    {data.updated_at}
                </Text>
            </Group>

            <Group>
                <Button onClick={() => onLoadClick(data)}>Load</Button>
            </Group>
        </Card>
    );
}
