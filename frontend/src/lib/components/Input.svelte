<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';

	let props: HTMLInputAttributes = $props();
</script>

<input class="input {props.class}" {...props} />
