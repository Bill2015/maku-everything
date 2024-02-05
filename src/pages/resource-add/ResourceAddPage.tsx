import { useState } from 'react';
import { Grid, Highlight, ScrollArea, Stack } from '@mantine/core';
import { ResourceDisplay } from '@pages/resource-detail/components/ResourceDisplay';
import { TagQuery } from '@api/tag';
import { useActiveCategoryRedux } from '@store/global';
import { useTagComboSelectValue } from '@components/input';
import { SeparatorInput, TextItem } from './components';
import { TextTagMapperProvider, useSeparatorText, useTextTagMapperContext } from './hooks';

export function ResourceAddPageContent() {
    const { activeCategory } = useActiveCategoryRedux();
    const [text, setText] = useState<string>('D:\\GithubRepo\\maku everything\\dataset\\hololive\\irys-[hololive]-holoen-thumb-anime-girl-gif-69930183607745831126993018360774583112');

    const { separators, setSeparators, separateResult, reset } = useSeparatorText(text);

    const { highlightText } = useTextTagMapperContext();

    const { data: tagData } = TagQuery.useGetByCategory(activeCategory.id);
    const tagOptionValues = useTagComboSelectValue(tagData);

    return (
        <Grid>
            <Grid.Col p={0} span={{ lg: 6, sm: 12 }} mah="100%" ta="center" display="flex" style={{ justifyContent: 'center' }}>
                <ResourceDisplay name="Hi" />
            </Grid.Col>
            <Grid.Col span={{ lg: 6, sm: 12 }} h="100%">
                <Stack>
                    <Highlight highlight={highlightText}>{text}</Highlight>
                    <SeparatorInput value={separators} onChange={setSeparators} onReset={reset} />
                    <ScrollArea.Autosize mx="auto" mah="470px" type="hover">
                        <Grid gutter="xs" pr={20}>
                            {
                                separateResult.map((val) => (
                                    <TextItem key={val} text={val} tagValues={tagOptionValues} />
                                ))
                            }
                        </Grid>
                    </ScrollArea.Autosize>
                </Stack>
            </Grid.Col>
        </Grid>
    );
}

export default function ResourceAddPage() {
    return (
        <TextTagMapperProvider>
            <ResourceAddPageContent />
        </TextTagMapperProvider>
    );
}
