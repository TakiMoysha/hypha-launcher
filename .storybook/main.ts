import type { StorybookConfig } from "@storybook/vue3-vite";
import vueJsx from "@vitejs/plugin-vue-jsx";
import tailwindcss from "@tailwindcss/vite";

const config: StorybookConfig = {
	stories: ["../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"],
	addons: [],
	framework: "@storybook/vue3-vite",
	async viteFinal(config) {
		return {
			...config,
			plugins: [...(config.plugins || []), vueJsx()],
		};
	},
};
export default config;
