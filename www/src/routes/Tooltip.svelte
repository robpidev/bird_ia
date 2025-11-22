<script lang="ts">
	let { content, children } = $props();
</script>

<div>
	<span> {content} </span>
	{@render children()}
</div>

<style>
	div {
		display: flex;
		position: relative;
	}
	span {
		/* position: absolute; */
		position: absolute;
		display: none;
		bottom: 0;
		right: 0;
		white-space: nowrap;
		transform: translateY(40px);
		background-color: rgba(255, 255, 255, 1);
		color: rgba(0, 0, 0, 1);
		border-radius: 6px;
		padding: 0.2rem 0.5rem;
		font-size: 0.5rem;
	}

	div:hover span {
		display: block;
	}
</style>
