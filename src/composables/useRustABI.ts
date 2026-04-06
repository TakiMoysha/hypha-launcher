import { invoke } from "@tauri-apps/api/core";

import * as Sentry from "@sentry/vue";

export default () => {
	return {
		runGameClient: (args: string[]) => {
			invoke("greeting_format", { name: "TEST" });
		},
	};
};
