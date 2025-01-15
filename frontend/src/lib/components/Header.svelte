<script lang="ts">
	import Button from './Button.svelte';

	let { onlogin, onsignup }: { onlogin?: () => void; onsignup?: () => void } = $props();
</script>

<header class="flex w-full flex-row place-content-between border-b-2 border-gray-200 px-3 pb-2">
	<div>
		<h2 class="m-0">Vexelstrom</h2>
	</div>

	<div>
		<Button
			onclick={() => {
				onlogin ? onlogin() : {};
			}}
		>
			Login
		</Button>

		<Button
			primary={false}
			onclick={() => {
				onsignup ? onsignup() : {};
			}}
		>
			Sign Up
		</Button>
	</div>
</header>
