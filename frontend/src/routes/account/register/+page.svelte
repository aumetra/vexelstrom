<script lang="ts">
	import Button from '$lib/components/Button.svelte';

	type AlertType = 'success' | 'error';

	let loading = $state(false);

	let alertData: { message: string; type: AlertType; show: boolean } = $state({
		message: '',
		type: 'success',
		show: false
	});

	async function onregister(e: SubmitEvent): Promise<void> {
		e.preventDefault();

		loading = true;

		const data = new FormData(e.target as HTMLFormElement);
		const username = data.get('username')!.toString();
		const email = data.get('email')!.toString();
		const password = data.get('password')!.toString();
		const confirmPassword = data.get('confirm-password')!.toString();

		if (password !== confirmPassword) {
			alertData.message = "Passwords aren't equal";
			alertData.type = 'error';
			alertData.show = true;

			loading = false;
			return;
		}

		const bodyData = new URLSearchParams({
			username,
			email,
			password,
			confirm_password: confirmPassword
		}).toString();

		try {
			const response = await fetch('/api/v1/register', {
				method: 'POST',
				body: bodyData
			});

			if (response.status === 201) {
				alertData.message = 'Registration successful!';
				alertData.type = 'success';
				alertData.show = true;

				loading = false;
				return;
			}

			const body = await response.text();
			alertData.message = `${body}`;
			alertData.type = 'error';
			alertData.show = true;

			loading = false;
		} catch (error) {
			alertData.message = `${error}`;
			alertData.type = 'error';
			alertData.show = true;

			loading = false;
		}
	}
</script>

<div class="hero min-h-screen">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<h1>Register account</h1>

			{#if alertData.show}
				<div role="alert" class="alert alert-{alertData.type}">
					<span>{alertData.message}</span>
				</div>
			{/if}

			<div class="card bg-base-300 rounded-box mt-5 grid place-items-center p-10">
				<form class="flex flex-col gap-3" onsubmit={onregister}>
					<input
						type="text"
						name="username"
						class="input validator"
						placeholder="Username"
						required
						disabled={loading}
					/>
					<input
						type="email"
						name="email"
						class="input validator"
						placeholder="Email address"
						required
						disabled={loading}
					/>
					<input
						type="password"
						name="password"
						class="input validator"
						placeholder="Password"
						required
						disabled={loading}
					/>
					<input
						type="password"
						name="confirm-password"
						class="input validator"
						placeholder="Confirm password"
						required
						disabled={loading}
					/>

					<p>
						<Button class="w-full" {loading}>Register</Button>
					</p>
				</form>
			</div>
		</div>
	</div>
</div>
