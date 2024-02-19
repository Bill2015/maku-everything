import { useEffect, useState, Suspense } from 'react';
import { useRoutes } from 'react-router-dom';
import { Notifications } from '@mantine/notifications';
import { Box, MantineProvider, AppShell } from '@mantine/core';
import { ContextMenuProvider } from 'mantine-contextmenu';

import { invoke } from '@tauri-apps/api/tauri';

import { MainNavbar } from '@components/navbar';
import { CreateSubjectModal } from '@modals/subject';
import { CreateTagModal } from '@modals/tag';
import { CreateResourceModal } from '@modals/resource';
import { ImportCategoryModal, CreateCategoryModal } from '@modals/category';
import { MainHeader } from '@components/header';
import { useViewportSize } from '@mantine/hooks';

import { ROUTE_OBJECTS } from './router/RoutingTable';

// https://mantine.dev/styles/mantine-styles/#css-layers
import '@mantine/core/styles.layer.css';
import '@mantine/dates/styles.layer.css';
import 'mantine-contextmenu/styles.layer.css';

import '@mantine/dropzone/styles.css';
import '@mantine/notifications/styles.css';
import './styles.css';

import classes from './App.module.scss';

function App() {
    const routes = useRoutes(ROUTE_OBJECTS);
    const [theme, setTheme] = useState<boolean>(false);
    const [isConnected, setIsConnected] = useState<boolean>(false);
    const { height, width } = useViewportSize();

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
            <ContextMenuProvider>
                <AppShell
                    classNames={{ main: classes.main }}
                    header={{ height: 40 }}
                    navbar={{
                        width:      70,
                        breakpoint: 'sm',
                        collapsed:  { mobile: true },
                    }}
                    padding="md"
                >
                    <AppShell.Header display="flex" style={{ alignItems: 'center' }}>
                        <MainHeader />
                    </AppShell.Header>

                    <AppShell.Navbar p="sm">
                        <MainNavbar />
                    </AppShell.Navbar>

                    <AppShell.Main w={width - 20} h={height}>
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
                <CreateCategoryModal />
                <ImportCategoryModal />
            </ContextMenuProvider>
        </MantineProvider>
    );
}

export default App;
