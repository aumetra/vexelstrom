<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import Input from '$lib/components/Input.svelte';

	const { Story } = defineMeta({
		title: 'Input',
		component: Input,
		tags: ['autodocs'],
		args: {
			oninput: fn()
		}
	});
</script>

<Story name="Enabled" args={{ disabled: false, placeholder: 'Hello world' }} />
<Story name="Disabled" args={{ disabled: true, placeholder: 'Hello world' }} />
