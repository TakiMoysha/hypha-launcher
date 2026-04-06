import "./assets/css/main.css";

import App from "./App.vue";

import { createApp } from "vue";
import { createPinia } from "pinia";
import { createRouter, createWebHashHistory } from "vue-router";

import * as Sentry from "@sentry/vue";

const pinia = createPinia();
const router = createRouter({
	history: createWebHashHistory(),
	routes: [
		{ name: "home", path: "/", component: import("./pages/HomeMain.vue") },
		{
			name: "settings:mods",
			path: "/settings/mods",
			component: import("./pages/WorkInProgress.vue"),
			children: [
				{
					name: "settings:mods:details",
					path: ":id",
					component: import("./pages/WorkInProgress.vue"),
				},
			],
		},
		// errors
		{
			name: "error:not-found",
			path: "/error/not-found",
			component: import("./pages/NotFound.vue"),
		},
		{
			name: "not-found",
			path: "/:catchAll(.*)*",
			redirect: { name: "error:not-found" },
		},
		// dev-server
		{
			name: "dev:index",
			path: "/dev",
			component: import("./pages/DevMain.vue"),
		},
		{
			name: "dev:server",
			path: "/dev/server",
			component: import("./pages/DevServerMain.vue"),
		},
		{
			name: "dev:inspect",
			path: "/dev/inspect",
			component: import("./pages/WorkInProgress.vue"),
		},
		// debug (not for production)
		{
			name: "debug:style-preview",
			path: "/debug/style-preview",
			component: import("./components/StylePreview.vue"),
		},
	],
});

const app = createApp(App);

Sentry.init({
	app,
	dsn: "https://908ba8b3b89918214bc7bef03819f077@o4511135509708800.ingest.de.sentry.io/4511135533170768",
	integrations: [
		Sentry.consoleLoggingIntegration({ levels: ["log", "warn", "error"] }),
	],
	sendDefaultPii: true,
	enableLogs: true,
});

app.use(pinia).use(router).mount("#app");
