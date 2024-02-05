import React from 'react';
import { RouteObject } from 'react-router-dom';
import { CategoryContainer } from '@pages/common';

const CategoryListPage = React.lazy(() => import(/* webpackChunkName: "category-page" */ '@pages/category-list/CategoryListPage'));
const ResourceListPage = React.lazy(() => import(/* webpackChunkName: "resources-page" */ '@pages/resource-list/ResourceListPage'));
const ResourcesDetailPage = React.lazy(() => import(/* webpackChunkName: "resources-detail-page" */ '@pages/resource-detail/ResourceDetailPage'));
const ResourceAddPage = React.lazy(() => import(/* webpackChunkName: "resources-add-page" */ '@pages/resource-add/ResourceAddPage'));

export const ROUTE_OBJECTS: RouteObject[] = [
    {
        path:    '/',
        element: <CategoryListPage />,
    },
    {
        path:     '/category/:categoryName/*',
        element:  <CategoryContainer />,
        children: [
            {
                path:    '*',
                element: <ResourceListPage />,
            },
            {
                path:    'resource/add',
                element: <ResourceAddPage />,
            },
            {
                path:    'resource/:resourceId',
                element: <ResourcesDetailPage />,
            },
        ],
    },
];

// export const ROUTE_OBJECTS: RouteObject[] = [
//     { path: '/', element: <Navigate to="/login" /> },
//     { path: 'login', element: <LoginPage /> },
//     { path: 'register', element: <RegisterPage /> },
// ];
