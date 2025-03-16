<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	let {
		children,
		primary = true,
		loading = false,
		...props
	}: { children: Snippet; primary?: boolean; loading?: boolean } & HTMLButtonAttributes = $props();
</script>

<button {...props} class="btn {props.class}" class:btn-primary={primary} disabled={loading}>
	{#if loading}
		<span class="loading loading-spinner"></span>
	{:else}
		{@render children()}
	{/if}
</button>
