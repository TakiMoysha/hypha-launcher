import { reactive } from "vue";
import { defineStore } from "pinia";

const settings = reactive({
	args: "",
	setArgs: (args: string) => (settings.args = args),
});

const appState = defineStore("app-data", () => {
	const appName = "hypha-launcher";
	const modList = [
		{
			id: "hypha-launcher",
			name: "Hypha Launcher",
			version: "0.1.0",
		},
	];
	return { appName, modList };
});

/** Loading settings from tauri.rs (system files/cache) when loading app and consistently updating */
export default () => {};
