<script lang="ts">
	import Button from './Button.svelte';

	let { onlogin, onsignup }: { onlogin?: () => void; onsignup?: () => void } = $props();
</script>

<header>
	<div class="bg-base-200 absolute flex w-full flex-row place-content-between p-5">
		<div>
			<h2 class="m-0"><a class="not-prose" href="/">Vexelstrom</a></h2>
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
	</div>
</header>
