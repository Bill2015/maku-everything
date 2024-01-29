import {
    Card, Group, Text, Badge, Button, rem, Spoiler, Box, Title, Divider, ActionIcon, Menu,
} from '@mantine/core';
import { CgExport } from 'react-icons/cg';
import { MdOutlineMoreVert } from 'react-icons/md';
import { IoSettingsOutline } from 'react-icons/io5';
import { LuPin } from 'react-icons/lu';

import { CategoryMutation, CategoryResDto } from '@api/category';

import { showNotification } from '@components/notification';
import { useActiveCategoryRedux } from '@store/global';
import { useCategoryNavigate } from '@router/navigateHook';
import { useCallback } from 'react';
import classes from './CategoryCard.module.scss';

export interface CategoryCardProps {
    data: CategoryResDto;
}

export function CategoryCard(props: CategoryCardProps) {
    const { data } = props;
    const { setActiveCategory } = useActiveCategoryRedux();
    const navigateCategoryTo = useCategoryNavigate();
    const exportCategory = CategoryMutation.useExport();

    // on load category
    const handleLoadClick = useCallback(async () => {
        showNotification('Loaded Category', data.name);
        setActiveCategory({ id: data.id, name: data.name });
        navigateCategoryTo(data.name);
    }, [data, setActiveCategory, navigateCategoryTo]);

    // on export click
    const handleExportClick = useCallback(async () => {
        await exportCategory.mutateAsync({ id: data.id });
    }, [exportCategory, data]);

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
                <Button onClick={handleLoadClick}>Load</Button>
            </Group>

            <Menu
                shadow="md"
                withArrow
                loop
                width={200}
                position="bottom-start"
                arrowSize={14}
                offset={0}
            >
                <Menu.Target>
                    <ActionIcon pos="absolute" right="10px" variant="outline" classNames={{ root: classes.menuBtn }}>
                        <MdOutlineMoreVert />
                    </ActionIcon>
                </Menu.Target>
                <Menu.Dropdown>
                    <Menu.Label>
                        {`${data.name}`}
                    </Menu.Label>
                    <Menu.Item leftSection={<IoSettingsOutline />}>
                        Settings
                    </Menu.Item>
                    <Menu.Item leftSection={<LuPin />}>
                        Pin
                    </Menu.Item>
                    <Menu.Item leftSection={<CgExport />} onClick={handleExportClick}>
                        Export
                    </Menu.Item>
                </Menu.Dropdown>
            </Menu>

        </Card>
    );
}
