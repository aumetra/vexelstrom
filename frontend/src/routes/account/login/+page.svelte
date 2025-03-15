<script lang="ts">
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
						<button class="btn btn-primary w-full" onclick={onlogin} disabled={loading}>
							{#if loading}
								<span class="loading loading-spinner"></span>
							{:else}
								Login
							{/if}
						</button>
					</p>
				</form>
			</div>
		</div>
	</div>
</div>
