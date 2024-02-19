/* eslint-disable react/jsx-props-no-spreading */
import { PropsWithChildren, useCallback, useEffect, useState } from 'react';
import {
    Divider, Group, Input, NumberInput, NumberInputProps, Slider, Stack, Switch, Text, Title,
} from '@mantine/core';
import { DateInput } from '@mantine/dates';
import { SubTitle } from '@components/display';
import { TagAttrPayload, TagCreateDto } from '@api/tag';
import { useTranslation } from 'react-i18next';
import { normalizeKey, useValueTranslation } from '@modules/i18next';

export interface TagAttributePanelRootProps extends PropsWithChildren {
    hidden: boolean;
}
function TagAttributePanelRoot(props: TagAttributePanelRootProps) {
    const { hidden, children } = props;
    return (
        <>
            {!hidden && <Divider orientation="vertical" />}
            <Stack display={hidden ? 'none' : 'flex'} w="35%">
                {children}
            </Stack>
        </>
    );
}

function TagAttributePanelNumber(props: {
    value: TagAttrPayload.Number;
    onStartChange: (value: number) => void;
    onEndChange: (value: number) => void;
    onDefaultChange: (value: number) => void;
}) {
    const { value, onStartChange, onEndChange, onDefaultChange } = props;
    const { t } = useTranslation('modal', { keyPrefix: 'createTag.TagAttributePanelContent' });

    const defaultProps: NumberInputProps = {
        w:                '7.5rem',
        size:             'xs',
        stepHoldDelay:    500,
        stepHoldInterval: (step) => Math.max(1000 / step ** 2, 25),
    };

    return (
        <>
            <Stack gap={5}>
                <SubTitle>{t('number_range')}</SubTitle>
                <Group>
                    <NumberInput
                        {...defaultProps}
                        label={t('number_start')}
                        min={0}
                        max={value.end}
                        defaultValue={0}
                        value={value.start}
                        onChange={(e) => {
                            const num = e as number;
                            onStartChange(num);
                            onDefaultChange(Math.max(num, value.defval));
                        }}
                    />
                    <NumberInput
                        {...defaultProps}
                        label={t('number_end')}
                        min={value.start}
                        max={Number.MAX_SAFE_INTEGER}
                        defaultValue={100}
                        value={value.end}
                        onChange={(e) => {
                            const num = e as number;
                            onEndChange(num);
                            onDefaultChange(Math.min(num, value.defval));
                        }}
                    />
                </Group>
            </Stack>

            <Stack gap={5}>
                <SubTitle>{t('default_value')}</SubTitle>
                <Slider
                    color="blue"
                    min={value.start}
                    max={value.end}
                    value={value.defval}
                    defaultValue={50}
                    onChange={onDefaultChange}
                />
            </Stack>
        </>
    );
}
// ======================================================================
function TagAttributePanelText(props: {
    value: TagAttrPayload.Text;
    onChange(value: string): void;
}) {
    const { value, onChange } = props;
    const { t } = useTranslation('modal', { keyPrefix: 'createTag.TagAttributePanelContent' });

    return (
        <Stack gap={3}>
            <SubTitle>{t('default_value')}</SubTitle>
            <Input
                placeholder={t('text_placehoder')}
                size="sm"
                w="100%"
                value={value.defval}
                onChange={(e) => onChange(e.currentTarget.value)}
            />
        </Stack>
    );
}

// ======================================================================
function TagAttributePanelBool(props: {
    value: TagAttrPayload.Bool;
    onChange(value: boolean): void;
}) {
    const { value, onChange } = props;
    const { t } = useTranslation('modal', { keyPrefix: 'createTag.TagAttributePanelContent' });

    return (
        <Stack gap={3}>
            <SubTitle>{t('default_value')}</SubTitle>
            <Switch
                color="lime"
                size="md"
                w="100%"
                label={value.defval ? t('bool_true') : t('bool_false')}
                checked={value.defval}
                onChange={(e) => onChange(e.currentTarget.checked)}
            />
        </Stack>
    );
}
// ======================================================================
function TagAttributePanelDate(props: {
    value: TagAttrPayload.Date;
    onChange(value: string): void;
}) {
    const { value, onChange } = props;
    const { t } = useTranslation('modal', { keyPrefix: 'createTag.TagAttributePanelContent' });

    function toDate() {
        const date = new Date(value.defval);
        return Number.isNaN(date.getTime()) ? null : date;
    }

    return (
        <Stack gap={3}>
            <SubTitle>{t('default_value')}</SubTitle>
            <DateInput
                w="100%"
                value={toDate()}
                onChange={(val) => val && onChange(val.toISOString())}
                valueFormat="YYYY/MM/DD"
                placeholder={t('date_placehoder')}
            />
        </Stack>
    );
}

// ======================================================================
export interface TagAttributePanelContentProps {
    displayType: TagCreateDto['tag_type'];

    onAttributeChange: (value: TagAttrPayload.All) => void;
}

function TagAttributePanelContent(props: TagAttributePanelContentProps) {
    const { displayType, onAttributeChange } = props;
    const { t } = useTranslation('modal', { keyPrefix: 'createTag.TagAttributePanelContent' });
    const { tv } = useValueTranslation('AttributeType');
    const [attrVal, setAttrVal] = useState<TagAttrPayload.All>({});

    // reset
    useEffect(() => setAttrVal(TagAttrPayload.DEFAULT_VALUE[displayType]), [displayType]);

    // emit on change event
    // eslint-disable-next-line react-hooks/exhaustive-deps
    useEffect(() => onAttributeChange(attrVal), [attrVal]);

    // update attributes
    const hanldeUpdate = <T extends TagAttrPayload.Variant, F extends keyof TagAttrPayload.AsType<T> = keyof TagAttrPayload.AsType<T>>(
        fieldName: F,
        value: TagAttrPayload.AsType<T>[F],
    ) => {
        setAttrVal((prev) => ({ ...prev, [fieldName]: value }));
    };

    const renderContent = useCallback(() => {
        switch (displayType) {
        case 'normal': return (<>empty</>);
        case 'number': return (
            <TagAttributePanelNumber
                value={TagAttrPayload.As<'number'>(attrVal)}
                onStartChange={(val) => hanldeUpdate<'number'>('start', val)}
                onEndChange={(val) => hanldeUpdate<'number'>('end', val)}
                onDefaultChange={(val) => hanldeUpdate<'number'>('defval', val)}
            />
        );
        case 'text': return (
            <TagAttributePanelText
                value={TagAttrPayload.As<'text'>(attrVal)}
                onChange={(val) => hanldeUpdate<'text'>('defval', val)}
            />
        );
        case 'date': return (
            <TagAttributePanelDate
                value={TagAttrPayload.As<'date'>(attrVal)}
                onChange={(val) => hanldeUpdate<'date'>('defval', val)}
            />
        );
        case 'bool': return (
            <TagAttributePanelBool
                value={TagAttrPayload.As<'bool'>(attrVal)}
                onChange={(val) => hanldeUpdate<'bool'>('defval', val)}
            />
        );
        default: return <Text>Needed implments</Text>;
        }
    }, [attrVal, displayType]);

    return (
        <Stack gap={15}>
            <Title order={5}>
                {t('title', { type: tv(normalizeKey(displayType)) })}
            </Title>
            {renderContent()}
        </Stack>
    );
}

export function TagAttributePanel() {}
TagAttributePanel.Root = TagAttributePanelRoot;
TagAttributePanel.Content = TagAttributePanelContent;
