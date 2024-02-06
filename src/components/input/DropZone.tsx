import { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Container, Center, Group, Text } from '@mantine/core';
import { appWindow } from '@tauri-apps/api/window';
import { UnlistenFn } from '@tauri-apps/api/event';

import classes from './DropZone.module.scss';

/**
 * The WebView feature Drag and Drop will conflict with Tauri config `fileDropEnabled` \
 * When disable the `fileDropEnabled` config, \
 * The WebView Drag and Drop event will not working \
 * But only Tauri drop feature can provide the **absolute path of file**, the WebView API can't.
 *
 * Solution:
 *  - Maybe can use another webview to drag and drop files
 *
 * Related Issues:
 *  - https://github.com/tauri-apps/tauri/discussions/4736
 *  - https://github.com/tauri-apps/tauri/issues/2768
 */

interface TauriDropZoneProps {
    onDropFiles: (filePaths: string[]) => void;
}

export function TauriDropZone(props: TauriDropZoneProps) {
    const { onDropFiles } = props;
    const { t } = useTranslation('common', { keyPrefix: 'Input.DropZone' });
    const dropListener = useRef<Promise<UnlistenFn>>();
    const [isDragHover, setDragHover] = useState<boolean>(false);

    useEffect(() => {
        if (dropListener && dropListener.current) {
            dropListener.current.then((fn) => fn());
        }
        dropListener.current = appWindow.onFileDropEvent((event) => {
            if (event.payload.type === 'hover' && event.payload.paths.length > 0) {
                setDragHover(true);
            }
            else if (event.payload.type === 'cancel') {
                setDragHover(false);
            }
            else {
                setDragHover(false);
                onDropFiles(event.payload.paths);
            }
        });

        // cleaned up
        return () => {
            if (dropListener && dropListener.current) {
                dropListener.current.then((fn) => fn());
            }
        };
    }, [onDropFiles]);

    return (
        <Center className={classes.dropzone} display={isDragHover ? 'flex' : 'none'}>
            <Container>
                <Group>
                    <div>
                        <Text size="xl" inline>
                            {t('title')}
                        </Text>
                        <Text size="sm" c="dimmed" inline mt={7}>
                            {t('sub_title')}
                        </Text>
                    </div>
                </Group>
            </Container>
        </Center>
    );
}
