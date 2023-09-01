import { PropsWithChildren } from 'react';
import { ActionIcon, Badge, Flex, Group, Stack, Text, rem } from '@mantine/core';
import { RxCross1 } from 'react-icons/rx';
import { ResourceTagDto } from '@api/resource';

export interface ResourceTagGroupProps {
    subjectName: string;

    subjectId: string;

    tagData: ResourceTagDto[];
}

export function ResourceTagGroup(props: ResourceTagGroupProps) {
    const { subjectName, subjectId, tagData } = props;

    const itemChip = tagData.map((val) => (
        <Badge
            pr={3}
            variant="outline"
            tt="initial"
            rightSection={(
                <ActionIcon size="xs" color="blue" radius="xl" variant="transparent">
                    <RxCross1 size={rem(10)} />
                </ActionIcon>
            )}
        >
            {val.name}
        </Badge>
    ));

    return (
        <Flex direction="column">
            <Text fz="md" c="indigo">{subjectName}</Text>
            <Group>
                {itemChip}
            </Group>
        </Flex>
    );
}

export interface ResourceTagStackProps extends PropsWithChildren { }

export function ResourceTagStack(props: ResourceTagStackProps) {
    const { children } = props;

    // Create Tag Map <SubjectName, TagData>

    // const tagGroupItem = useMemo(() => {
    //     const item: ReactNode[] = [];
    //     for (const [key, val] of tagMap.entries()) {
    //         item.push(
    //             <ResourceTagGroup subjectId="" subjectName={key} tagData={val} />,
    //         );
    //     }
    //     return item;
    // }, [tagMap]);

    return (
        <Stack>
            {children}
        </Stack>
    );
}

ResourceTagStack.Group = ResourceTagGroup;
