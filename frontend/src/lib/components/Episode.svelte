<script lang="ts">
	import Icon from '@iconify/svelte';

	let {
		cover,
		title,
		description,
		onplay
	}: { cover: string; title: string; description: string; onplay?: () => void } = $props();

	let hovered = $state(false);
</script>

<div class="flex w-96 flex-row rounded-lg border border-gray-400">
	<figure class="relative m-0 w-32 *:h-full *:rounded-l-lg">
		<img class="w-full" src={cover} alt="Cover" />

		<figcaption
			class="absolute left-0 top-0 m-0 w-full bg-black opacity-0 transition hover:bg-opacity-50 hover:opacity-100"
		>
			<button
				class="h-full w-full"
				onclick={() => {
					onplay ? onplay() : {};
				}}
			>
				<Icon width="80" class="m-auto text-white" icon="mdi:play" />
			</button>
		</figcaption>
	</figure>

	<div class="flex flex-col justify-center px-2">
		<h3 class="m-0 p-0">
			{title}
		</h3>

		<p class="mt-1 leading-tight">
			{description}
		</p>
	</div>
</div>
