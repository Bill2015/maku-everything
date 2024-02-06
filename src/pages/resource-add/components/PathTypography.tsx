import { Button, Highlight, Stack } from '@mantine/core';
import { useContextMenu } from 'mantine-contextmenu';
import { IoAddOutline } from 'react-icons/io5';
import { useTextSelection } from '@mantine/hooks';
import { useTextTagMapperContext } from '../hooks';

import classes from './PathTypography.module.scss';

const contextmenuOption = { classNames: { root: classes.contextmenuRoot } };

export interface PathTypographyProps {
    text: string;

    highlight: string;
}

export function PathTypography(props: PathTypographyProps) {
    const { text, highlight } = props;
    const { showContextMenu } = useContextMenu();
    const { textMap, textMapInsert } = useTextTagMapperContext();
    const selection = useTextSelection();

    const contextmenu = (close: () => void) => (
        <Stack gap={0}>
            <Button
                className={classes.menuItem}
                leftSection={<IoAddOutline />}
                onClick={() => {
                    close();
                    const selectionText = selection?.toString();
                    if (!selectionText || textMap.has(selectionText)) {
                        return;
                    }
                    textMapInsert(selectionText, null);
                }}
            >
                Add To Rule
            </Button>
        </Stack>
    );

    return (
        <Highlight
            highlight={highlight}
            onContextMenu={showContextMenu(contextmenu, contextmenuOption)}
            style={{ wordBreak: 'break-all' }}
        >
            {text}
        </Highlight>
    );
}
