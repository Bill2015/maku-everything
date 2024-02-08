import { useEffect, useRef } from 'react';
import { Blockquote, Button, Group, Highlight, Popover, Stack, Text } from '@mantine/core';
import { useContextMenu } from 'mantine-contextmenu';
import { useDisclosure } from '@mantine/hooks';
import { IoAddOutline } from 'react-icons/io5';
import { FaRegLightbulb } from 'react-icons/fa';

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
    const textRef = useRef<HTMLDivElement>(null);
    const { showContextMenu } = useContextMenu();
    const [hintOpened, { close: closeHint, open: openHint }] = useDisclosure(false);

    useEffect(() => {
        setTimeout(() => openHint(), 500);
        setTimeout(() => {
            closeHint();
        }, 4000);
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

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
                Add To Rule
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
            <Popover width="50%" opened={hintOpened} position="bottom" withArrow shadow="md">
                <Popover.Target>
                    <Highlight
                        ref={textRef}
                        highlight={highlight}
                        onContextMenu={showContextMenu(contextmenu, contextmenuOption)}
                        style={{ wordBreak: 'break-all' }}
                    >
                        {text.replace(rootPath, '')}
                    </Highlight>
                </Popover.Target>

                <Popover.Dropdown>
                    <Blockquote color="blue" p={0} pl={40} iconSize={30} icon={<FaRegLightbulb />}>
                        You can select those text and
                        <strong> Rigth Click </strong>
                        to add tag rules
                    </Blockquote>
                </Popover.Dropdown>
            </Popover>

        </Group>
    );
}
