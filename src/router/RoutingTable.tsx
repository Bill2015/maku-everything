import { Navigate, RouteObject, Outlet } from 'react-router-dom';
import { Box } from '@mantine/core';

import { CategoriesPage } from '@pages/category';
import { ResourcesPage } from '@pages/resource';

export const ROUTE_OBJECTS: RouteObject[] = [
    {
        path:    '/*',
        element: <CategoriesPage />,
    },
    {
        path:    'category/:categoryName',
        element: <ResourcesPage />,
    },
];

// export const ROUTE_OBJECTS: RouteObject[] = [
//     { path: '/', element: <Navigate to="/login" /> },
//     { path: 'login', element: <LoginPage /> },
//     { path: 'register', element: <RegisterPage /> },
// ];
