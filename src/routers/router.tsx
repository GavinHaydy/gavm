import { createBrowserRouter, Navigate } from "react-router-dom";
import { GlobalLayout } from "../layouts/DefaultLayout";
import { PythonPage } from "@/pages/python";
import { SettingPage } from "@/pages/setting";
import { ErrorPage } from "@/pages/error";

export const router = createBrowserRouter([
    {
        path: "/",
        element: <GlobalLayout/>,
        errorElement: <ErrorPage />,
        children: [
            {
                index: true,
                element: <Navigate to="/python"/>
            },
            {
                path: "python",
                element: <PythonPage/>
            },
            {
                path: "settings",
                element: <SettingPage/>
            }
        ]
    }
])