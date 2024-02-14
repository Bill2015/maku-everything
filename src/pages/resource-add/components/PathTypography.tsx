import { useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { Button, Group, Highlight, Stack, Text } from '@mantine/core';
import { useContextMenu } from 'mantine-contextmenu';
import { IoAddOutline } from 'react-icons/io5';

import classes from './PathTypography.module.scss';

const contextmenuOption = { classNames: { root: classes.contextmenuRoot } };

export interface PathTypographyProps {
    rootPath: string;

    text: string;

    highlight: string;

    onClickAddRule: () => void;
}

export function PathTypography(props: PathTypographyProps) {
    const { rootPath, text, highlight, onClickAddRule } = props;
    const { t } = useTranslation('pages', { keyPrefix: 'resourceAdd.PathTypography' });
    const textRef = useRef<HTMLDivElement>(null);
    const { showContextMenu } = useContextMenu();

    const contextmenu = (close: () => void) => (
        <Stack gap={0}>
            <Button
                className={classes.menuItem}
                leftSection={<IoAddOutline />}
                onClick={() => {
                    close();
                    onClickAddRule();
                }}
            >
                {t('add_to_rule')}
            </Button>
        </Stack>
    );

    return (
        <Group gap={5} align="baseline">
            {
                text.startsWith(rootPath)
                    ? <Text title={rootPath} opacity={0.5} fz="xs">local:\\</Text>
                    : <Text opacity={0.5} fz="xs">url:\\</Text>
            }

            <Highlight
                ref={textRef}
                highlight={highlight}
                onContextMenu={showContextMenu(contextmenu, contextmenuOption)}
                style={{ wordBreak: 'break-all' }}
            >
                {text.replace(rootPath, '')}
            </Highlight>
        </Group>
    );
}
