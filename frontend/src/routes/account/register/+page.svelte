<script lang="ts">
	let loading = $state(false);

	function onregister(e: Event) {
		e.preventDefault();

		loading = true;
	}
</script>

<div class="hero min-h-screen">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<h1>Register account</h1>

			<div class="card bg-base-300 rounded-box grid place-items-center p-10">
				<form class="flex flex-col gap-3">
					<input type="username" class="input" placeholder="Username" disabled={loading} />
					<input type="email" class="input" placeholder="Email address" disabled={loading} />
					<input type="password" class="input" placeholder="Password" disabled={loading} />
					<input type="password" class="input" placeholder="Confirm password" disabled={loading} />

					<p>
						<button class="btn btn-primary w-full" onclick={onregister} disabled={loading}>
							{#if loading}
								<span class="loading loading-spinner"></span>
							{:else}
								Register
							{/if}
						</button>
					</p>
				</form>
			</div>
		</div>
	</div>
</div>
