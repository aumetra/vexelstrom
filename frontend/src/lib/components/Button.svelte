<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	let {
		children,
		primary = true,
		...props
	}: { children: Snippet; primary?: boolean } & HTMLButtonAttributes = $props();
</script>

<button
	class="rounded-lg px-4 py-2"
	class:bg-purple-500={primary}
	class:bg-transparent={!primary}
	class:text-white={primary}
	class:border={!primary}
	{...props}
>
	{@render children()}
</button>
