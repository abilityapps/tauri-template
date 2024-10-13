import { RouterProvider, createRouter } from '@tanstack/react-router'

import React from "react";
import ReactDOM from "react-dom/client";
import { ThemeProvider } from "./components/theme-provider";
import { routeTree } from './routeTree.gen'

// Set up a Router instance
const router = createRouter({
	routeTree,
	defaultPreload: 'intent',
})
  
// Register things for typesafety
declare module '@tanstack/react-router' {
	interface Register {
		router: typeof router
	}
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<ThemeProvider defaultTheme="light" storageKey="vite-ui-theme">
			<RouterProvider router={router} />
		</ThemeProvider>
	</React.StrictMode>,
);
