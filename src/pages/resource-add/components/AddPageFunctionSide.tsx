import { Collapse, Stack, Tabs, Title } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { IoIosArrowForward } from 'react-icons/io';
import { BsGear } from 'react-icons/bs';
import { LiaMapSignsSolid } from 'react-icons/lia';
import { LuTableProperties } from 'react-icons/lu';

import { TagQuery } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';
import { useTagComboSelectValue } from '@components/input';
import { PathTypography } from './PathTypography';
import { useTextTagMapperContext } from '../hooks';
import { TagMapperDisplayer } from './TagMapperDisplayer';

import classes from './AddPageFunctionSide.module.scss';

export interface AddPageFunctionSideProps {
    rootPath: string;

    text: string;
}

export function AddPageFunctionSide(props: AddPageFunctionSideProps) {
    const { rootPath, text } = props;
    const { activeCategory } = useActiveCategoryRedux();
    const { textMap, highlightText } = useTextTagMapperContext();
    const { data: tagData } = TagQuery.useGetByCategory(activeCategory.id);
    const tagValues = useTagComboSelectValue(tagData);
    const [opened, { toggle }] = useDisclosure(false);

    return (
        <Stack mah="100%">
            <PathTypography rootPath={rootPath} text={text} highlight={highlightText} />
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
                    <Title order={4} c="dimmed" onClick={toggle}>
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
