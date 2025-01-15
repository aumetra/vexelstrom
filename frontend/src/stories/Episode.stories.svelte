<script lang="ts" module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Episode from '$lib/components/Episode.svelte';
	import EpisodeCover from './assets/episode-cover.png';

	const { Story } = defineMeta({
		title: 'Episode',
		component: Episode,
		tags: ['autodocs']
	});
</script>

<Story
	name="Default"
	args={{
		cover: EpisodeCover,
		title: 'Example Episode',
		description: 'This is a very riveting episode from the series of "Example"'
	}}
/>
