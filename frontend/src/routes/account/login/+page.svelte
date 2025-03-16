<script lang="ts">
	import Button from '$lib/components/Button.svelte';

	let loading = $state(false);

	function onlogin(e: Event) {
		e.preventDefault();

		loading = true;
	}
</script>

<div class="hero min-h-screen">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<h1>Log into account</h1>

			<div class="card bg-base-300 rounded-box grid place-items-center p-10">
				<form class="flex flex-col gap-3">
					<input type="username" class="input" placeholder="Username" disabled={loading} />
					<input type="password" class="input" placeholder="Password" disabled={loading} />

					<p>
						<Button class="w-full" onclick={onlogin} {loading}>Login</Button>
					</p>
				</form>
			</div>
		</div>
	</div>
</div>
