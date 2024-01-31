import {
    Card, Group, Text, Button, Tooltip, Divider, Accordion, Stack,
} from '@mantine/core';
import { ResourceResDto } from '@api/resource';
import { DateTimeDisplayer, LinkIcon, ResourceThumbnailDisplayer } from '@components/display';

import classes from './ResourceCard.module.scss';

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
                            top="5px"
                            right="5%"
                            host={data.url!.host}
                            url={data.url!.full}
                        />
                    )
                }
                <ResourceThumbnailDisplayer data={data} />
            </Card.Section>
            <Accordion defaultValue="" classNames={{ content: classes.accordioncontent }}>
                <Accordion.Item value={data.name}>
                    <Accordion.Control p={0} h="28px">
                        <Tooltip label={data.name} openDelay={500}>
                            <Text fw={500} truncate="end">{data.name}</Text>
                        </Tooltip>
                    </Accordion.Control>
                    <Accordion.Panel p={0} style={{ zIndex: 9999 }}>
                        <Divider />
                        <Text pt="xs">{data.description}</Text>

                        <Stack gap="xs">
                            <DateTimeDisplayer label="Created At:" date={data.created_at} />
                            <DateTimeDisplayer label="Updated At:" date={data.updated_at} />

                            <Group justify="flex-end">
                                <Button onClick={() => onDetailClick(data)}>Detail</Button>
                            </Group>
                        </Stack>
                    </Accordion.Panel>
                </Accordion.Item>
            </Accordion>
        </Card>
    );
}
