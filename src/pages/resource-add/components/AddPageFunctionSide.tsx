import { Button, Collapse, Stack, Tabs, Title } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { IoIosArrowForward } from 'react-icons/io';
import { BsGear } from 'react-icons/bs';
import { LiaMapSignsSolid } from 'react-icons/lia';
import { LuTableProperties } from 'react-icons/lu';

import { TagQuery } from '@api/tag';
import { useTagComboSelectValue } from '@components/input';
import { CategoryResDto } from '@api/category';
import { PathTypography } from './PathTypography';
import { ActiveResourceType, useTextTagMapperContext } from '../hooks';
import { TagMapperDisplayer } from './TagMapperDisplayer';

import classes from './AddPageFunctionSide.module.scss';

export interface AddPageFunctionSideProps {
    category: CategoryResDto | null;

    activeResource: ActiveResourceType;
}

export function AddPageFunctionSide(props: AddPageFunctionSideProps) {
    const { category, activeResource } = props;
    const { textMap, highlightText } = useTextTagMapperContext();
    const { data: tagData } = TagQuery.useGetByCategory(category?.id || '');
    const tagValues = useTagComboSelectValue(tagData);

    const [opened, { toggle }] = useDisclosure(false);

    if (!activeResource) {
        return (
            <Stack>
                <Title order={3}>Settings</Title>
                <Button variant="outline" bg="transparent" fz="md" fw={700} c="dimmed" onClick={toggle}>
                    <IoIosArrowForward />
                    Global Defined Tag Map
                </Button>
                <Collapse in={opened} display="grid" mih={0}>
                    <TagMapperDisplayer
                        texts={Array.from(textMap.keys())}
                        tagValues={tagValues}
                    />
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
            />
            <Tabs defaultValue="tag" classNames={{ root: classes.tabRoot, panel: classes.tabPanel }}>
                <Tabs.List>
                    <Tabs.Tab value="tag" leftSection={<LiaMapSignsSolid />}>
                        Tag
                    </Tabs.Tab>
                    <Tabs.Tab value="attr" leftSection={<LuTableProperties />}>
                        Attr
                    </Tabs.Tab>
                    <Tabs.Tab value="settings" ml="auto" leftSection={<BsGear />}>
                        Settings
                    </Tabs.Tab>
                </Tabs.List>

                <Tabs.Panel value="tag">
                    <TagMapperDisplayer
                        texts={Array.from(textMap.keys()).filter((val) => text.toLowerCase().includes(val.toLowerCase()))}
                        tagValues={tagValues}
                    />
                </Tabs.Panel>

                <Tabs.Panel value="settings" p={10}>
                    <Title variant="outline" bg="transparent" fz="md" fw={700} c="dimmed" onClick={toggle}>
                        <IoIosArrowForward />
                        Global Defined Tag Map
                    </Title>
                    <Collapse in={opened} display="grid" mih={0}>
                        <TagMapperDisplayer
                            texts={Array.from(textMap.keys())}
                            tagValues={tagValues}
                        />
                    </Collapse>
                </Tabs.Panel>
            </Tabs>
        </Stack>
    );
}
