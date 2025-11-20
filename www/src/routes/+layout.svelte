<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="container">
	{@render children()}
</div>

<style>
	.container {
		padding: 1rem;
		display: flex;
		flex-direction: column;
		max-width: 1200px;
		justify-self: center;
		gap: 1rem;
	}
</style>
