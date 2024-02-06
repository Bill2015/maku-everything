import { useTranslation } from 'react-i18next';
import { Card, Group, Text, Button, Tooltip, Accordion, Stack } from '@mantine/core';
import { ResourceResDto } from '@api/resource';
import { DateTimeDisplayer, LinkIcon, ResourceThumbnailDisplayer } from '@components/display';

import classes from './ResourceCard.module.scss';

export interface ResourceCardProps {
    data: ResourceResDto;

    onDetailClick: (data: ResourceResDto) => void;
}

export function ResourceCard(props: ResourceCardProps) {
    const { data, onDetailClick } = props;
    const { t } = useTranslation('pages', { keyPrefix: 'resourceList.ResourceCard' });

    return (
        <Card shadow="sm" padding="lg" radius="md" withBorder classNames={{ root: classes.cardroot }}>
            <Card.Section>
                {
                    data.url && (
                        <LinkIcon
                            pos="absolute"
                            top="5px"
                            right="5%"
                            url={data.url!}
                        />
                    )
                }
                <ResourceThumbnailDisplayer url={data.url?.full} filePath={`${data.root_path}${data.file?.path}`} alt={data.name} />
            </Card.Section>
            <Accordion defaultValue="" classNames={{ content: classes.acccontent }}>
                <Accordion.Item value={data.name}>
                    <Accordion.Control p={0} h="28px">
                        <Tooltip label={data.name} openDelay={500}>
                            <Text fw={500} truncate="end">{data.name}</Text>
                        </Tooltip>
                    </Accordion.Control>
                    <Accordion.Panel p={0} style={{ zIndex: 9999 }}>
                        <Text pt="xs">{data.description}</Text>

                        <Stack gap="xs">
                            <DateTimeDisplayer label={t('created_at')} date={data.created_at} />
                            <DateTimeDisplayer label={t('updated_at')} date={data.updated_at} />

                            <Group justify="flex-end">
                                <Button onClick={() => onDetailClick(data)}>{t('detail')}</Button>
                            </Group>
                        </Stack>
                    </Accordion.Panel>
                </Accordion.Item>
            </Accordion>
        </Card>
    );
}
