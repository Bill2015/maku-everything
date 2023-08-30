import React from 'react';
import { Provider } from 'react-redux';
import { QueryClientProvider, QueryClient } from '@tanstack/react-query';

import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';

import { store } from '@store/store';

import App from './App';
import './styles.css';

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <BrowserRouter>
            <Provider store={store}>
                <QueryClientProvider client={queryClient}>
                    <App />
                </QueryClientProvider>
            </Provider>
        </BrowserRouter>
    </React.StrictMode>,
);
