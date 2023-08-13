import { Navigate, RouteObject, Outlet } from 'react-router-dom';


// export const ROUTE_OBJECTS: RouteObject[] = [
//     { path: '/', element: <Navigate to="/login" /> },
//     { path: 'login', element: <LoginPage /> },
//     { path: 'register', element: <RegisterPage /> },
//     {
//         path:    ':user/*',
//         element: (
//             <RequireUserToken>
//                 <GeekyMainApp />
//             </RequireUserToken>
//         ),
//         children: [
//             { path: 'project', element: <ProjectList /> },
//             { path: 'createProject', element: <CreateProjectPage /> },
//             {
//                 path:    ':projectID/*',
//                 element: (
//                     <RequireProject>
//                         <Outlet />
//                     </RequireProject>
//                 ),
//                 children: [
//                     { path: 'file', element: <FileViewer /> },
//                     { path: 'management', element: <ProjectPermissionPage /> },
//                     { path: 'search', element: <SearchViewer /> },
//                     { path: 'documents/new', element: <CreateDocumentViewer /> },
//                     { path: 'documents/:documentID', element: <SingleDocumentViewer /> },
//                     { path: 'documents', element: <DocumentListViewer /> },
//                     { path: 'coverage', element: <CoverageViewer /> },
//                     { path: 'labels/new', element: <CreateLabelViewer /> },
//                     { path: 'labels/:labelId', element: <SingleLabelViewer /> },
//                     { path: 'labels', element: <LabelListViewer /> },
//                     { path: 'trace', element: <TraceCaseLabelListViewer /> },
//                     { path: 'coupling', element: <CouplingLabelListViewer /> },
//                     { path: '*', element: <NoMatch /> },
//                 ],
//             },
//         ],
//     },
// ];


// export const ROUTE_OBJECTS: RouteObject[] = [
//     { path: '/', element: <Navigate to="/login" /> },
//     { path: 'login', element: <LoginPage /> },
//     { path: 'register', element: <RegisterPage /> },
// ];

