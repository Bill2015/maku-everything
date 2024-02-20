import { ResourceTagAttrValDto, ResourceTagDto } from '@api/resource';
import { EditableText } from '@components/display';
import { Group, Pill, Text } from '@mantine/core';
import millify from 'millify';

import classes from './ResourceTagPill.module.scss';

export interface ResourceTagPillProps {
    tag: ResourceTagDto

    onRemoveTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;

    onUpdateTag: (tag: Pick<ResourceTagDto, 'id'|'name'>, value: ResourceTagAttrValDto) => void;
}

export function ResourceTagPill(props: ResourceTagPillProps) {
    const { tag, onRemoveTag, onUpdateTag } = props;

    return (
        <Pill
            classNames={classes}
            withRemoveButton
            key={tag.id}
            onRemove={() => onRemoveTag({ id: tag.id, name: tag.name })}
        >
            <Group gap={3} align="baseline">
                {tag.name}
                {tag.attrval !== null && (
                    <Group gap={0} align="baseline">
                        <Text c="teal" fz="0.8rem">[</Text>
                        <EditableText c="teal" fz="0.8rem" value={tag.attrval?.toString()} name="attr" onChange={(val) => onUpdateTag({ id: tag.id, name: tag.name }, val)} />
                        <Text c="teal" fz="0.8rem">]</Text>
                    </Group>
                )}
                <Text size="xs" opacity="0.6" component="span">
                    {`(${millify(tag.tagged_count)})`}
                </Text>
            </Group>
        </Pill>
    );
}
