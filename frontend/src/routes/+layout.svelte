<script lang="ts">
	import { goto } from '$app/navigation';
	import Header from '$lib/components/Header.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<Header onlogin={() => goto('/account/login')} onsignup={() => goto('/account/register')} />

{@render children()}
