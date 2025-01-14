<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import Button from '$lib/components/Button.svelte';

	const { Story } = defineMeta({
		title: 'Button',
		component: Button,
		tags: ['autodocs'],
		args: {
			onclick: fn()
		}
	});
</script>

<!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
{#snippet template(args: any)}
	<Button primary={args.primary ?? true} onclick={args.onclick}>Hello world</Button>
{/snippet}

<Story name="Primary" args={{ primary: true }} children={template} />
<Story name="Secondary" args={{ primary: false }} children={template} />
