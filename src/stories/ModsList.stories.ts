import type { Meta, StoryObj } from '@storybook/vue3-vite';

// import { expect, userEvent, within } from 'storybook/test';

import ModsTable from '../components/ui/ModsTable.vue';

const meta = {
  title: 'Components/ModsTable',
  component: ModsTable,
  render: () => ({
    components: { ModsTable },
    template: '<mods-table />',
  }),
  parameters: {
    // More on how to position stories at: https://storybook.js.org/docs/configure/story-layout
    layout: 'fullscreen',
  },
  // This component will have an automatically generated docsPage entry: https://storybook.js.org/docs/writing-docs/autodocs
  tags: ['autodocs'],
} satisfies Meta<typeof ModsTable>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on component testing: https://storybook.js.org/docs/writing-tests/interaction-testing
export const Default: Story = {};

// export const LoggedIn: Story = {
//   play: async ({ canvasElement }: any) => {
//     const canvas = within(canvasElement);
//     const loginButton = canvas.getByRole('button', { name: /Log in/i });
//     await expect(loginButton).toBeInTheDocument();
//     await userEvent.click(loginButton);
//     await expect(loginButton).not.toBeInTheDocument();
//
//     const logoutButton = canvas.getByRole('button', { name: /Log out/i });
//     await expect(logoutButton).toBeInTheDocument();
//   },
// };
//
// export const LoggedOut: Story = {};
