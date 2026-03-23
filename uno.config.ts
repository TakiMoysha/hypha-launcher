import { defineConfig, presetWind4 } from "unocss";

export default defineConfig({
	presets: [
		presetWind4({
			preflights: {
				reset: true,
			},
		}),
	],

	rules: [
    ['text-primary', { color: 'var(--color-primary)' }],
    ['text-primary-content', { color: 'var(--color-primary-content)' }],
    ['text-secondary', { color: 'var(--color-secondary)' }],
    ['text-secondary-content', { color: 'var(--color-secondary-content)' }],
    ['text-accent', { color: 'var(--color-accent)' }],
    ['text-accent-content', { color: 'var(--color-accent-content)' }],
    ['text-base-content', { color: 'var(--color-base-content)' }],
    ['text-info', { color: 'var(--color-info)' }],
    ['text-info-content', { color: 'var(--color-info-content)' }],
    ['text-success', { color: 'var(--color-success)' }],
    ['text-success-content', { color: 'var(--color-success-content)' }],
    ['text-warning', { color: 'var(--color-warning)' }],
    ['text-warning-content', { color: 'var(--color-warning-content)' }],
    ['text-error', { color: 'var(--color-error)' }],
    ['text-error-content', { color: 'var(--color-error-content)' }],

    ['bg-primary', { 'background-color': 'var(--color-primary)' }],
    ['bg-primary-content', { 'background-color': 'var(--color-primary-content)' }],
    ['bg-secondary', { 'background-color': 'var(--color-secondary)' }],
    ['bg-secondary-content', { 'background-color': 'var(--color-secondary-content)' }],
    ['bg-accent', { 'background-color': 'var(--color-accent)' }],
    ['bg-accent-content', { 'background-color': 'var(--color-accent-content)' }],
    ['bg-base-100', { 'background-color': 'var(--color-base-100)' }],
    ['bg-base-200', { 'background-color': 'var(--color-base-200)' }],
    ['bg-base-300', { 'background-color': 'var(--color-base-300)' }],
    ['bg-info', { 'background-color': 'var(--color-info)' }],
    ['bg-success', { 'background-color': 'var(--color-success)' }],
    ['bg-warning', { 'background-color': 'var(--color-warning)' }],
    ['bg-error', { 'background-color': 'var(--color-error)' }],

    ['border-primary', { 'border-color': 'var(--color-primary)' }],
    ['border-secondary', { 'border-color': 'var(--color-secondary)' }],
    ['border-accent', { 'border-color': 'var(--color-accent)' }],
    ['border-info', { 'border-color': 'var(--color-info)' }],
    ['border-success', { 'border-color': 'var(--color-success)' }],
    ['border-warning', { 'border-color': 'var(--color-warning)' }],
    ['border-error', { 'border-color': 'var(--color-error)' }],

    ['focus-ring-primary', { 'box-shadow': '0 0 0 2px var(--color-primary-focus)' }],
    ['focus-ring-secondary', { 'box-shadow': '0 0 0 2px var(--color-secondary-focus)' }],
    ['focus-ring-accent', { 'box-shadow': '0 0 0 2px var(--color-accent-focus)' }],
	],
});
