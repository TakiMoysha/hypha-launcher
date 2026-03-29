import "./assets/css/main.css";

import App from "./App.vue";

import { createApp } from "vue";
import { createPinia } from "pinia";
import { createRouter, createWebHashHistory } from "vue-router";

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

		{
			name: "dev",
			path: "/dev",
			component: import("./pages/DebugScreen.vue"),
			children: [
				// {
				// 	name: "dev:style-preview",
				// 	path: "style-preview",
				// 	component: import("./components/StylePreview.vue"),
				// },
			],
		},

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
	],
});

createApp(App).use(pinia).use(router).mount("#app");
