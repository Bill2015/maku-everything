import { useEffect, useState, Suspense } from 'react';
import { useRoutes } from 'react-router-dom';
import { Notifications } from '@mantine/notifications';
import { Box, MantineProvider, AppShell, Affix, Button } from '@mantine/core';

import { invoke } from '@tauri-apps/api/tauri';

import { MainNavbar } from '@components/navbar';
import { CreateSubjectModal } from '@modals/subject';
import { CreateTagModal } from '@modals/tag';
import { CreateResourceModal } from '@modals/resource';
import { ImportCategoryModal } from '@modals/category';
import { MainHeader } from '@components/header';

import { ROUTE_OBJECTS } from './router/RoutingTable';

// https://mantine.dev/styles/mantine-styles/#css-layers
import '@mantine/core/styles.layer.css';

import '@mantine/dropzone/styles.css';
import '@mantine/notifications/styles.css';
import classes from './App.module.scss';
import { useTranslation } from 'react-i18next';
import { useToggle } from '@mantine/hooks';

function App() {
    const routes = useRoutes(ROUTE_OBJECTS);
    const [theme, setTheme] = useState<boolean>(false);
    const [isConnected, setIsConnected] = useState<boolean>(false);
    const [lang, toggle] = useToggle(['enUS', 'zhTW']);
    const { i18n } = useTranslation();

    const changeLanguage = () => {
        console.log(lang);
        toggle();
        i18n.changeLanguage(lang);
    };

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

                <AppShell.Main pos="relative" display="flex">
                    <Suspense fallback={<Box>FallBack</Box>}>
                        {routes}
                    </Suspense>
                </AppShell.Main>

                <AppShell.Footer h={20}>
                    <div>Hi</div>
                </AppShell.Footer>
            </AppShell>
            <Affix position={{ bottom: 20, left: 20 }}>
                <Button onClick={changeLanguage}>Hi</Button>
            </Affix>
            <Notifications classNames={{ notification: classes.notification }} />
            <CreateSubjectModal />
            <CreateTagModal />
            <CreateResourceModal />
            <ImportCategoryModal />
        </MantineProvider>
    );
}

export default App;
