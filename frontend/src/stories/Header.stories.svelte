<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Header from '$lib/components/Header.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'Header',
		component: Header,
		tags: ['autodocs'],
		args: {
			onlogin: fn(),
			onsignup: fn()
		}
	});
</script>

<Story name="Logged out" />
