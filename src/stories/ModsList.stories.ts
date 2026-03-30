import type { Meta, StoryObj } from "@storybook/vue3-vite";

// import { expect, userEvent, within } from 'storybook/test';

import ModsTable from "../components/ui/ModsTable.vue";

const meta = {
  title: "Components/ModsTable",
  component: ModsTable,
  tags: ["autodocs"],
} satisfies Meta<typeof ModsTable>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
