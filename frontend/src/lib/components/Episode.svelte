<script lang="ts">
	let { cover, title, description }: { cover: string; title: string; description: string } =
		$props();
</script>

<div class="flex w-96 flex-row place-items-center rounded-lg border border-gray-400">
	<img class="m-0 w-32 rounded-l-lg" src={cover} alt="Cover" />

	<div class="px-2">
		<h3 class="m-0 p-0">
			{title}
		</h3>

		<p class="mt-1 leading-tight">
			{description}
		</p>
	</div>
</div>
