<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';

	let props: HTMLInputAttributes = $props();
</script>

<input
	class="border-0 border-b-2 border-gray-200 bg-transparent"
	class:cursor-not-allowed={props.disabled}
	class:text-gray-200={props.disabled}
	{...props}
/>
