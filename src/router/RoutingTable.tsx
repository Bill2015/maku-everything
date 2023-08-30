import React from 'react';
import { RouteObject } from 'react-router-dom';

const CategoriesPage = React.lazy(() => import(/* webpackChunkName: "category-page" */ '@pages/category/CategoriesPage'));
const ResourcePage = React.lazy(() => import(/* webpackChunkName: "resources-page" */ '@pages/resource/ResourcesPage'));
const ResourcesDetailPage = React.lazy(() => import(/* webpackChunkName: "resources-detail-page" */ '@pages/resource-detail/ResourceDetailPage'));

export const ROUTE_OBJECTS: RouteObject[] = [
    {
        path:    '/',
        element: <CategoriesPage />,
    },
    {
        path:    '/category/:categoryName/*',
        element: <ResourcePage />,
    },
    {
        path:    '/category/:categoryName/resource/:resourceId',
        element: <ResourcesDetailPage />,
    }
];

// export const ROUTE_OBJECTS: RouteObject[] = [
//     { path: '/', element: <Navigate to="/login" /> },
//     { path: 'login', element: <LoginPage /> },
//     { path: 'register', element: <RegisterPage /> },
// ];
