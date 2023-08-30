import { useEffect, useState, Suspense } from 'react';
import { Provider } from 'react-redux';
import { QueryClientProvider, QueryClient } from '@tanstack/react-query';
import { SnackbarProvider } from 'notistack';
import { Box, MantineProvider, Header, AppShell } from '@mantine/core';
import { useRoutes } from 'react-router-dom';

import { invoke } from '@tauri-apps/api/tauri';

import { MainNavbar } from '@components/navbar';
import { store } from '@store/store';
import { ROUTE_OBJECTS } from './router/RoutingTable';
import './App.css';

const queryClient = new QueryClient();

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

    async function dbTest() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        console.log(await invoke('db_test2'));
    }

    return (
        <Provider store={store}>
            <QueryClientProvider client={queryClient}>
                <SnackbarProvider anchorOrigin={{ horizontal: 'right', vertical: 'bottom' }}>
                    <MantineProvider withGlobalStyles withNormalizeCSS theme={{ colorScheme: theme ? 'dark' : 'light' }}>
                        <AppShell
                            padding="md"
                            navbar={<MainNavbar />}
                            header={<Header height={60} p="xs">{/* Header content */}</Header>}
                            styles={(theme) => ({ main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0] } })}
                        >
                            <Suspense fallback={<Box>FallBack</Box>}>
                                {routes}
                            </Suspense>
                        </AppShell>
                    </MantineProvider>
                </SnackbarProvider>
            </QueryClientProvider>
        </Provider>
    );
}

export default App;
