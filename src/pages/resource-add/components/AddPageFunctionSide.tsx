import { Button, Collapse, Stack, Tabs, Title } from '@mantine/core';
import { useDisclosure, useTextSelection } from '@mantine/hooks';
import { IoIosArrowForward } from 'react-icons/io';
import { BsGear } from 'react-icons/bs';
import { LiaMapSignsSolid } from 'react-icons/lia';
import { BiDetail } from 'react-icons/bi';

import { TagQuery } from '@api/tag';
import { useTagComboSelectValue } from '@components/input';
import { useAddResourceContext, useTextTagMapperContext } from '../stores';
import { PathTypography } from './PathTypography';
import { TagMapperDisplayer } from './TagMapperDisplayer';
import { AttributePanel } from './AttributePanel';

import classes from './AddPageFunctionSide.module.scss';

export function AddPageFunctionSide() {
    const { category, activeResource } = useAddResourceContext();

    const { highlightText, checkTextExist, textMapInsert } = useTextTagMapperContext();
    const { data: tagData } = TagQuery.useGetByCategory(category?.id || '');
    const tagValues = useTagComboSelectValue(tagData);

    const [opened, { toggle }] = useDisclosure(false);
    const selection = useTextSelection();

    if (!activeResource) {
        return (
            <Stack>
                <Title order={3}>Settings</Title>
                <Button variant="outline" bg="transparent" fz="md" fw={700} c="dimmed" onClick={toggle}>
                    <IoIosArrowForward />
                    Global Defined Tag Map
                </Button>
                <Collapse in={opened} display="grid" mih={0}>
                    <TagMapperDisplayer global tagValues={tagValues} />
                </Collapse>
            </Stack>
        );
    }

    const text = activeResource.data.file_path! || activeResource.data.url_path!;

    return (
        <Stack mah="100%">
            <PathTypography
                rootPath={category?.root_path || ''}
                text={text}
                highlight={highlightText}
                onClickAddRule={() => {
                    // prevent select other elements text
                    if (selection?.getRangeAt(0).commonAncestorContainer.nodeType !== Node.TEXT_NODE) {
                        return;
                    }
                    const selectionText = selection?.toString();
                    if (!selectionText || checkTextExist(selectionText)) {
                        return;
                    }
                    textMapInsert(selectionText, null);
                }}
            />
            <Tabs defaultValue="tag" classNames={{ root: classes.tabRoot, panel: classes.tabPanel }}>
                <Tabs.List>
                    <Tabs.Tab value="tag" leftSection={<LiaMapSignsSolid />}>
                        Tag
                    </Tabs.Tab>
                    <Tabs.Tab value="attr" leftSection={<BiDetail />}>
                        Attrs
                    </Tabs.Tab>
                    <Tabs.Tab value="settings" ml="auto" leftSection={<BsGear />}>
                        Settings
                    </Tabs.Tab>
                </Tabs.List>

                <Tabs.Panel value="tag">
                    <TagMapperDisplayer targetText={text} tagValues={tagValues} />
                </Tabs.Panel>

                <Tabs.Panel value="attr">
                    <AttributePanel tagValues={tagValues} />
                </Tabs.Panel>

                <Tabs.Panel value="settings" p={10}>
                    <Title variant="outline" bg="transparent" fz="md" fw={700} c="dimmed" onClick={toggle}>
                        <IoIosArrowForward />
                        Global Defined Tag Map
                    </Title>
                    <Collapse in={opened} display="grid" mih={0}>
                        <TagMapperDisplayer global tagValues={tagValues} />
                    </Collapse>
                </Tabs.Panel>
            </Tabs>
        </Stack>
    );
}
