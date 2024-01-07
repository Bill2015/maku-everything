import { useEffect, useRef, useState } from 'react';
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
    const dropListener = useRef<UnlistenFn>();
    const [isDragHover, setDragHover] = useState<boolean>(false);

    useEffect(() => {
        async function dropEventRegister() {
            dropListener.current = await appWindow.onFileDropEvent((event) => {
                if (event.payload.type === 'hover') {
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
        }
        dropEventRegister();

        // cleaned up
        return () => {
            if (dropListener && dropListener.current) {
                dropListener.current();
            }
        };
    // prevent rebinding the drop event
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    return (
        <Center className={classes.dropzone} display={isDragHover ? 'flex' : 'none'}>
            <Container>
                <Group>
                    <div>
                        <Text size="xl" inline>
                            Drag File here or click to select files
                        </Text>
                        <Text size="sm" c="dimmed" inline mt={7}>
                            Attach as many files as you like
                        </Text>
                    </div>
                </Group>
            </Container>
        </Center>
    );
}
