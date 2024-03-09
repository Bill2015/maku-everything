/* eslint-disable react/jsx-props-no-spreading */
import { useCallback, useState } from 'react';
import millify from 'millify';
import { ResourceTagAttrValDto, ResourceTagDto } from '@api/resource';
import { EditableText } from '@components/display';
import { TagAttrPayload } from '@api/tag';
import { BoxProps, Group, Pill, Text } from '@mantine/core';
import { EditableBool, EditableDate, EditableNumber } from './attributes';

import classes from './ResourceTagPill.module.scss';

export interface ResourceTagPillProps {
    tag: ResourceTagDto

    onRemoveTag: (tag: Pick<ResourceTagDto, 'id'|'name'>) => void;

    onUpdateTag: (tag: Pick<ResourceTagDto, 'id'|'name'>, value: ResourceTagAttrValDto) => void;
}

export function ResourceTagPill(props: ResourceTagPillProps) {
    const { tag, onRemoveTag, onUpdateTag } = props;
    const [value, setValue] = useState<ResourceTagAttrValDto>(tag.attrval);

    const renderAttributeField = useCallback(() => {
        const baseProps: BoxProps & { name: string } = {
            c:    'teal',
            fz:   '0.8rem',
            name: tag.name,
        };

        switch (tag.tag_type) {
        case 'number': {
            const attr = tag.attr as TagAttrPayload.Number;
            return (
                <EditableNumber
                    {...baseProps}
                    value={value as number}
                    min={attr.start}
                    max={attr.end}
                    onChange={setValue}
                    onEditFinished={(val, isEdited) => {
                        if (isEdited) {
                            onUpdateTag({ id: tag.id, name: tag.name }, val);
                        }
                    }}
                />
            );
        }
        case 'text': {
            return (
                <EditableText
                    {...baseProps}
                    value={value as string}
                    onChange={setValue}
                    onEditFinished={(val, isEdited) => {
                        if (isEdited) {
                            onUpdateTag({ id: tag.id, name: tag.name }, val);
                        }
                    }}
                />
            );
        }
        case 'date':
            return (
                <EditableDate
                    {...baseProps}
                    value={value as string}
                    onChange={setValue}
                    onEditFinished={(val, isEdited) => {
                        if (isEdited) {
                            onUpdateTag({ id: tag.id, name: tag.name }, val);
                        }
                    }}
                />
            );
        case 'bool':
            return (
                <EditableBool
                    {...baseProps}
                    value={value as boolean}
                    onChange={(val) => {
                        setValue(val);
                        onUpdateTag({ id: tag.id, name: tag.name }, val);
                    }}
                />
            );
        default:
            break;
        }
    }, [onUpdateTag, tag, value]);

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
                    <Group gap={0} align="baseline" style={{ cursor: 'pointer' }}>
                        <Text c="teal" fz="0.8rem">[</Text>
                        {renderAttributeField()}
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
