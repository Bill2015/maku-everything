import { Divider, Group, ScrollArea, Stack, Text } from '@mantine/core';
import { TagQuery } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';
import { useTagComboSelectValue } from '@components/input';
import { PathTypography } from './PathTypography';
import { TextItem } from './TextItem';
import { useTextTagMapperContext } from '../hooks';

export interface AddPageFunctionSideProps {
    text: string;
}

export function AddPageFunctionSide(props: AddPageFunctionSideProps) {
    const { text } = props;
    const { activeCategory } = useActiveCategoryRedux();
    const { textMap, highlightText } = useTextTagMapperContext();
    const { data: tagData } = TagQuery.useGetByCategory(activeCategory.id);
    const tagOptionValues = useTagComboSelectValue(tagData);

    return (
        <Stack mah="100%">
            <PathTypography text={text} highlight={highlightText} />
            <Stack mih={0}>
                <Group>
                    <Text>Target Text</Text>
                    <Text>Appended Tag</Text>
                </Group>
                <ScrollArea.Autosize h="100%" style={{ textAlign: 'start' }}>
                    <Stack gap={10} pr={20}>
                        {
                            Array.from(textMap.keys()).map((val) => (
                                <>
                                    <TextItem key={val} text={val} tagValues={tagOptionValues} />
                                    <Divider opacity={0.25} />
                                </>
                            ))
                        }
                    </Stack>
                </ScrollArea.Autosize>
            </Stack>
        </Stack>
    );
}
