<script lang="ts">
	import Tooltip from './Tooltip.svelte';

	let { min, max, avg, gen }: { min: number; max: number; avg: number; gen: number } = $props();
</script>

<div>
	<span class="title">Stats: </span>
	<ul>
		<Tooltip content="Food min consumed in last generation">
			<li>
				<span class="label">Min:</span>
				<span class="value">{min}</span>
			</li>
		</Tooltip>

		<Tooltip content="Food max consumed in last generation">
			<li>
				<span class="label">Max:</span>
				<span class="value">{max}</span>
			</li>
		</Tooltip>

		<Tooltip content="Food Average consumed in last generation">
			<li>
				<span class="label">Avg:</span>
				<span class="value">{avg.toFixed(2)}</span>
			</li>
		</Tooltip>

		<Tooltip content="Last generation">
			<li>
				<span class="label">Gen:</span>
				<span class="value">{gen}</span>
			</li>
		</Tooltip>
	</ul>
</div>

<style>
	div {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title {
		font-weight: bold;
		font-size: 1.2rem;
	}
	ul {
		list-style-type: none;
		display: flex;
		gap: 1rem;
		margin: 0;
		padding: 0;
	}
	li {
		display: flex;
		gap: 1rem;
		background: rgba(255, 255, 255, 0.1);
		padding: 0.25rem 1rem;
		border-radius: 6px;
		cursor: default;
		user-select: none;
	}

	li:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	.label {
		font-weight: bold;
	}
	.value {
		font-weight: normal;
	}
</style>
