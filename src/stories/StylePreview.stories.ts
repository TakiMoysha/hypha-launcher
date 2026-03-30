import type { Meta, StoryObj } from '@storybook/vue3-vite';

import StylePreview from '../components/StylePreview.vue';

const meta = {
  title: 'Components/StylePreview',
  component: StylePreview,
  parameters: {
    // More on how to position stories at: https://storybook.js.org/docs/configure/story-layout
    layout: 'fullscreen',
  },
  // This component will have an automatically generated docsPage entry: https://storybook.js.org/docs/writing-docs/autodocs
  tags: ['autodocs'],
} satisfies Meta<typeof StylePreview>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on component testing: https://storybook.js.org/docs/writing-tests/interaction-testing
export const Default: Story = {};

