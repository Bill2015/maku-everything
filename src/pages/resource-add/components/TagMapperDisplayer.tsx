import { Fragment } from 'react';
import { Divider, Group, ScrollArea, Space, Stack, Text } from '@mantine/core';
import { TagSelectOptionValue } from '@components/input';
import { TagMapperItem } from './TagMapperItem';

export interface TagMapperDisplayerProps {
    texts: string[];

    tagValues: TagSelectOptionValue[];
}

export function TagMapperDisplayer(props: TagMapperDisplayerProps) {
    const { texts, tagValues } = props;

    return (
        <Stack mih={0} gap={0}>
            <Group align="center" pt="sm">
                <Text fw="bolder" c="violet" flex="0 0 30%">Target Text</Text>
                <Text fw="bolder" c="violet">Appended Tag</Text>
            </Group>
            <Divider />
            <ScrollArea.Autosize pt="sm" type="auto" style={{ textAlign: 'start' }}>
                <Stack gap={10} pr={20}>
                    {
                        texts.map((val, index) => (
                            <Fragment key={val}>
                                { (index > 0) && <Divider opacity={0.25} /> }
                                <TagMapperItem key={val} text={val} tagValues={tagValues} />
                            </Fragment>
                        ))
                    }
                    <Space />
                </Stack>
            </ScrollArea.Autosize>
        </Stack>
    );
}
