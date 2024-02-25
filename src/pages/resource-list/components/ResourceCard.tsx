import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FaCircleRight } from 'react-icons/fa6';
import { MdInfoOutline } from 'react-icons/md';
import {
    Card, Text, Tooltip, Stack, Collapse, ActionIcon, Box, UnstyledButton,
} from '@mantine/core';
import { useHover, useToggle } from '@mantine/hooks';

import { ResourceResDto } from '@api/resource';
import { ActionFileIcon, DateTimeDisplayer, ActionLinkIcon, ResourceThumbnailDisplayer } from '@components/display';

import classes from './ResourceCard.module.scss';

export interface ResourceCardProps {
    data: ResourceResDto;

    onDetailClick: (data: ResourceResDto) => void;
}

export function ResourceCard(props: ResourceCardProps) {
    const { data, onDetailClick } = props;
    const { t } = useTranslation('pages', { keyPrefix: 'resourceList.ResourceCard' });
    const [info, toggleInfo] = useToggle();
    const { hovered, ref: hoverRef } = useHover();
    const [tooltipOn, setTooltipOn] = useState<boolean>(false);

    return (
        <Tooltip label={data.name} disabled={tooltipOn} openDelay={500} color="gray">
            <Card ref={hoverRef} shadow="lg" radius="md" withBorder classNames={{ root: classes.cardroot }}>
                <Card.Section mb="-10px">
                    <Box pos="relative" display="ruby-text">
                        <Stack gap={0} pos="absolute" top="5px" right="5px">
                            {data.file && (
                                <ActionFileIcon variant="light" color="gray" fz="1.5rem" filePath={data.root_path + data.file.path} onTooltipChange={setTooltipOn} />
                            )}
                            {data.url && (
                                <ActionLinkIcon url={data.url!} onTooltipChange={setTooltipOn} />
                            )}
                        </Stack>
                        {hovered && (
                            <>
                                <ActionIcon pos="absolute" left="5px" bottom="10px" variant="light" onClick={() => toggleInfo()}>
                                    <MdInfoOutline />
                                </ActionIcon>
                                <ActionIcon pos="absolute" size="lg" right="5px" bottom="10px" variant="light" title={t('detail')} onClick={() => onDetailClick(data)}>
                                    <FaCircleRight />
                                </ActionIcon>
                            </>
                        )}
                        <UnstyledButton>
                            <ResourceThumbnailDisplayer url={data.url?.full} filePath={`${data.root_path}${data.file?.path}`} alt={data.name} />
                        </UnstyledButton>
                    </Box>
                </Card.Section>
                <Collapse in={info}>
                    <Text fw={500} truncate="end" pt="20px">{data.name}</Text>
                    <Text pt="xs">{data.description}</Text>

                    <Stack gap="xs" pb="20px">
                        <DateTimeDisplayer label={t('created_at')} date={data.created_at} />
                        <DateTimeDisplayer label={t('updated_at')} date={data.updated_at} />
                    </Stack>
                </Collapse>
            </Card>
        </Tooltip>
    );
}
