import NotFound from "../pages/NotFound.vue";
import type { Meta, StoryObj } from "@storybook/vue3-vite";

const meta = {
	title: "Screens/NotFound",
	component: NotFound,
	tags: ["autodocs"],
} satisfies Meta<typeof NotFound>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Preview: Story = {};
