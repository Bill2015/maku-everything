import { useState } from 'react';
import { Outlet } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import {
    ActionIcon, Box, Button, Collapse, Flex, Stack, Tabs, Tooltip,
} from '@mantine/core';
import { FaTags } from 'react-icons/fa';
import { MdKeyboardDoubleArrowLeft, MdOutlineKeyboardDoubleArrowRight } from 'react-icons/md';
import { RiFunctionLine } from 'react-icons/ri';
import { IoAdd } from 'react-icons/io5';

import { useActiveCategoryRedux } from '@store/global';
import { useCreateResourceModal, useCreateSubjectModal, useCreateTagModal } from '@store/modal';
import { useResourceAddNavigate } from '@router/navigateHook';

export function CategoryContainer() {
    const { activeCategory } = useActiveCategoryRedux();
    const { t } = useTranslation('pages', { keyPrefix: 'Common.CategoryContainer' });
    const navigateToAdd = useResourceAddNavigate();

    const [_subject, { open: openSubject }] = useCreateSubjectModal();
    const [_tag, { open: openTag }] = useCreateTagModal();
    const [_resource, { open: openResource }] = useCreateResourceModal();

    const iconStyle = { width: '1em', height: '1em' };
    const [isOpen, setOpen] = useState<boolean>(true);
    const [isCollapse, setIsCollapse] = useState<boolean>(true);

    return (
        <Flex>
            <Box style={{ flexGrow: 1 }} display="grid">
                <Outlet />
            </Box>
            <Collapse
                in={isOpen}
                transitionDuration={0}
                onTransitionEnd={() => setIsCollapse(!isCollapse)}
            >
                <Box w="200px">
                    <ActionIcon
                        onClick={() => setOpen(!isOpen)}
                        variant="outline"
                        pos="absolute"
                        right="8px"
                        style={{ zIndex: 9999 }}
                    >
                        <MdOutlineKeyboardDoubleArrowRight />
                    </ActionIcon>
                    <Tabs radius="md" defaultValue="tags">
                        <Tabs.List>
                            <Tooltip label={t('TagsPanel.title')} openDelay={500}>
                                <Tabs.Tab value="tags" leftSection={<FaTags style={iconStyle} />} />
                            </Tooltip>
                            <Tooltip label={t('DisplayPanel.title')} openDelay={500}>
                                <Tabs.Tab value="display" leftSection={<RiFunctionLine style={iconStyle} />} />
                            </Tooltip>
                        </Tabs.List>

                        <Tabs.Panel value="tags">
                            <Stack p={10}>
                                <Button onClick={() => openSubject()}>
                                    <IoAdd />
                                    {t('TagsPanel.subject')}
                                </Button>
                                <Button onClick={() => openTag()}>
                                    <IoAdd />
                                    {t('TagsPanel.tag')}
                                </Button>
                                <Button onClick={() => openResource()}>
                                    <IoAdd />
                                    {t('TagsPanel.resource')}
                                </Button>

                                <Button onClick={() => navigateToAdd(activeCategory.name)}>
                                    To Add
                                </Button>
                            </Stack>
                        </Tabs.Panel>

                        <Tabs.Panel value="display">
                            Messages tab content
                        </Tabs.Panel>
                    </Tabs>
                </Box>
            </Collapse>
            <Box
                pos="absolute"
                right="8px"
                style={{ zIndex: 9999 }}
                hidden={!isCollapse || isOpen}
            >
                <ActionIcon variant="outline" onClick={() => setOpen(!isOpen)}>
                    <MdKeyboardDoubleArrowLeft />
                </ActionIcon>
            </Box>
        </Flex>
    );
}
