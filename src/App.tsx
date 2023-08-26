import { useEffect, useState } from 'react';
import { Provider } from 'react-redux';
import { QueryClientProvider, QueryClient } from '@tanstack/react-query';
import { SnackbarProvider } from 'notistack';
import { Box, MantineProvider, Button, Header, AppShell } from '@mantine/core';
import { invoke } from '@tauri-apps/api/tauri';

import { MainNavbar } from '@components/navbar';
import { store } from '@store/store';
import { CategoriesPage } from './pages/category';
import './App.css';

const queryClient = new QueryClient();

function App() {
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
                            <Box>
                                <Button onClick={() => setTheme(!theme)}>Theme</Button>
                                { isConnected && <h1>Database Connected！</h1> }

                                <button
                                    type="button"
                                    onClick={() => {
                                        dbTest();
                                    }}
                                >
                                    DB Test
                                </button>
                                <CategoriesPage />
                            </Box>
                        </AppShell>
                    </MantineProvider>
                </SnackbarProvider>
            </QueryClientProvider>
        </Provider>
    );
}

export default App;
