import { useEffect, useState, Suspense } from 'react';
import { useRoutes } from 'react-router-dom';
import { Notifications } from '@mantine/notifications';
import { Box, MantineProvider, AppShell, Group } from '@mantine/core';

import { invoke } from '@tauri-apps/api/tauri';

import { MainNavbar } from '@components/navbar';
import { CreateSubjectModal } from '@modals/subject';
import { CreateTagModal } from '@modals/tag';
import { CreateResourceModal } from '@modals/resource';

import { ROUTE_OBJECTS } from './router/RoutingTable';
import { Initializer } from './__test__/components/Initializer';

import '@mantine/core/styles.css';
import '@mantine/dropzone/styles.css';
import '@mantine/notifications/styles.css';
import classes from './App.module.scss';

function App() {
    const routes = useRoutes(ROUTE_OBJECTS);
    const [theme, setTheme] = useState<boolean>(false);
    const [isConnected, setIsConnected] = useState<boolean>(false);

    useEffect(() => {
        if (isConnected === false) {
            invoke('connect_db')
                .then((value) => {
                    setIsConnected(true);
                })
                .catch(() => {
                    setIsConnected(true);
                });
        }
    }, [isConnected]);

    return (
        <MantineProvider defaultColorScheme="dark">
            <AppShell
                classNames={{ main: classes.main }}
                header={{ height: 60 }}
                navbar={{
                    width:      70,
                    breakpoint: 'sm',
                    collapsed:  { mobile: true },
                }}
                padding="md"
            >
                <AppShell.Header>
                    <Group px="md">
                        Header
                    </Group>
                </AppShell.Header>

                <AppShell.Navbar p="sm">
                    <MainNavbar />
                </AppShell.Navbar>

                <AppShell.Main mah="100vh" display="flex">
                    <Suspense fallback={<Box>FallBack</Box>}>
                        {routes}
                    </Suspense>
                </AppShell.Main>

                <AppShell.Footer h={20}>
                    <div>Hi</div>
                </AppShell.Footer>
            </AppShell>
            <Notifications classNames={{ notification: classes.notification }} />
            <CreateSubjectModal />
            <CreateTagModal />
            <CreateResourceModal />
            <Initializer />
        </MantineProvider>
    );
}

export default App;
